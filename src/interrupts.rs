use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::{halt_loop, println};

lazy_static! {
    static ref TABLE: InterruptDescriptorTable = {
        let mut table = InterruptDescriptorTable::new();
        table.divide_error.set_handler_fn(handle_divide_error);
        table.debug.set_handler_fn(handle_debug);
        table.non_maskable_interrupt.set_handler_fn(handle_non_maskable_interrupt);
        table.breakpoint.set_handler_fn(handle_breakpoint);
        table.overflow.set_handler_fn(handle_overflow);
        table.bound_range_exceeded.set_handler_fn(handle_bound_range_exceeded);
        table.invalid_opcode.set_handler_fn(handle_invalid_opcode);
        table.device_not_available.set_handler_fn(handle_device_not_available);
        table.double_fault.set_handler_fn(handle_double_fault);
        table.invalid_tss.set_handler_fn(handle_invalid_tss);
        table.segment_not_present.set_handler_fn(handle_segment_not_present);
        table.stack_segment_fault.set_handler_fn(handle_stack_segment_fault);
        table.general_protection_fault.set_handler_fn(handle_general_protection_fault);
        table.page_fault.set_handler_fn(handle_page_fault);
        table.x87_floating_point.set_handler_fn(handle_x87_floating_point);
        table.alignment_check.set_handler_fn(handle_alignment_check);
        table.machine_check.set_handler_fn(handle_machine_check);
        table.simd_floating_point.set_handler_fn(handle_simd_floating_point);
        table.virtualization.set_handler_fn(handle_virtualization);
        table.vmm_communication_exception.set_handler_fn(handle_vmm_communication);
        table.security_exception.set_handler_fn(handle_security);
        table
    };
}

pub fn init_idt() {
    TABLE.load()
}

extern "x86-interrupt" fn handle_divide_error(frame: InterruptStackFrame) {
    println!("Cannot divide by zero! {:?}", frame);
}

extern "x86-interrupt" fn handle_debug(frame: InterruptStackFrame) {
    println!("Debug: {:?}", frame);
}

extern "x86-interrupt" fn handle_non_maskable_interrupt(frame: InterruptStackFrame) {
    println!("Non-maskable interrupt! {:?}", frame);
}

extern "x86-interrupt" fn handle_breakpoint(frame: InterruptStackFrame) {
    println!("Breakpoint: {:?}", frame);
}

extern "x86-interrupt" fn handle_overflow(frame: InterruptStackFrame) {
    println!("Overflow! {:?}", frame);
}

extern "x86-interrupt" fn handle_bound_range_exceeded(frame: InterruptStackFrame) {
    println!("Bound range exceeded! {:?}", frame);
}

extern "x86-interrupt" fn handle_invalid_opcode(frame: InterruptStackFrame) {
    println!("Invalid opcode! {:?}", frame);
}

extern "x86-interrupt" fn handle_device_not_available(frame: InterruptStackFrame) {
    println!("Device not available! {:?}", frame);
}

extern "x86-interrupt" fn handle_double_fault(frame: InterruptStackFrame, error_code: u64) -> ! {
    println!("Double fault! {:?} (error code: {})", frame, error_code);
    halt_loop()
}

extern "x86-interrupt" fn handle_invalid_tss(frame: InterruptStackFrame, error_code: u64) {
    println!("Invalid TSS! {:?} (error code: {})", frame, error_code);
}

extern "x86-interrupt" fn handle_segment_not_present(frame: InterruptStackFrame, error_code: u64) {
    println!("Segment not present! {:?} (error code: {})", frame, error_code);
}

extern "x86-interrupt" fn handle_stack_segment_fault(frame: InterruptStackFrame, error_code: u64) {
    println!("Stack segment fault! {:?} (error code: {})", frame, error_code);
}

extern "x86-interrupt" fn handle_general_protection_fault(frame: InterruptStackFrame, error_code: u64) {
    println!("General protection fault! {:?} (error code: {})", frame, error_code);
}

extern "x86-interrupt" fn handle_page_fault(frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    println!("Page fault! {:?} (error code: {:?})", frame, error_code);
}

extern "x86-interrupt" fn handle_x87_floating_point(frame: InterruptStackFrame) {
    println!("Error with x87 floating point! {:?}", frame);
}

extern "x86-interrupt" fn handle_alignment_check(frame: InterruptStackFrame, error_code: u64) {
    println!("Alignment check failed! {:?} (error code: {})", frame, error_code);
}

extern "x86-interrupt" fn handle_machine_check(frame: InterruptStackFrame) -> ! {
    println!("Machine check failed! {:?}", frame);
    halt_loop()
}

extern "x86-interrupt" fn handle_simd_floating_point(frame: InterruptStackFrame) {
    println!("Error with SIMD floating point! {:?}", frame);
}

extern "x86-interrupt" fn handle_virtualization(frame: InterruptStackFrame) {
    println!("Virtualization error! {:?}", frame);
}

extern "x86-interrupt" fn handle_vmm_communication(frame: InterruptStackFrame, error_code: u64) {
    println!("VMM communication error! {:?} (error code: {})", frame, error_code);
}

extern "x86-interrupt" fn handle_security(frame: InterruptStackFrame, error_code: u64) {
    println!("Security error! {:?} (error code: {})", frame, error_code);
}
