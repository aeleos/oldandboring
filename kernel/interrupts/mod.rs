pub use x86_64::structures::idt::{ExceptionStackFrame, HandlerFunc, Idt};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;

use spin::Once;
use drivers;

use super::MEMORY_CONTROLLER;

mod gdt;
mod pic;
mod pit;

const NUM_IRQS: usize = 256 - 32;


lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }

        macro_rules! register_handler {
            ($handler:path, $int_num:expr) => {{
                #[allow(unused_variables)]
                extern "x86-interrupt" fn irq_handler(stack_frame: &mut ExceptionStackFrame) {
                    $handler();
                    notify_irq_eoi($int_num);
                }
                idt.interrupts[$int_num].set_handler_fn(irq_handler);
            }}
        }

        for i in 0..idt.interrupts.len() {
            idt.interrupts[i].set_handler_fn(irq_no_func_handler);
        }

        register_handler!(self::pit::irq_handler, 0);
        register_handler!(drivers::keyboard::ps2::irq_handler, 1);
        idt
    };
}

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

pub fn init() {
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::interrupts;

    let mut memory_controller = MEMORY_CONTROLLER.lock();

    let double_fault_stack = memory_controller
        .alloc_stack(1)
        .expect("could not allocate double fault stack");

    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] =
            VirtualAddress(double_fault_stack.top());
        tss
    });

    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);

    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        gdt
    });

    gdt.load();

    unsafe {
        // reload code segment register
        set_cs(code_selector);

        // load TSS
        load_tss(tss_selector);
    }

    IDT.load();

    // Initialize the PIC and enable interrupts (STI)
    unsafe {
        pic::initialize();
        pit::initialize();
        interrupts::enable();
    }
}

extern "x86-interrupt" fn irq_handler(stack_frame: &mut ExceptionStackFrame) {
    panic!(
        "IRQ with no handler was called, stack frame:\n{:#?}",
        stack_frame
    );
}

pub fn notify_irq_eoi(num_interrupt: u8) {
    unsafe {
        self::pic::notify_isr_eoi(num_interrupt + 32);
    }
}

const DOUBLE_FAULT_IST_INDEX: usize = 0;


extern "x86-interrupt" fn irq_no_func_handler(stack_frame: &mut ExceptionStackFrame) {
    panic!(
        "IRQ with no handler was called, stack frame:\n{:#?}",
        stack_frame
    );
}

#[allow(unused_variables)]
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    debugln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[allow(unused_variables)]
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    debugln!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}
