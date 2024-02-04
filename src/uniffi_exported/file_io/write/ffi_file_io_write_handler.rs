use crate::prelude::*;

#[uniffi::export(with_foreign)]
pub trait FFIFileIOWriteHandler: FFIOperationHandler<FFIFileIOWriteOutcomeListener> {
    /// Rust will tell the handler to execute `operation` by calling this
    /// function, which a concrete type FFI side (Swift side) has implemented.
    /// Once the operation has finished with a result (Success/Failure) it
    /// passes back the result using the `listener_rust_side` callback.
    fn execute_file_io_write_request(
        &self,
        request: FFIFileIOWriteRequest,
        listener_rust_side: Arc<FFIFileIOWriteOutcomeListener>,
    ) -> Result<(), FFISideError>;
}

impl<U: FFIFileIOWriteHandler> FFIOperationHandler<FFIFileIOWriteOutcomeListener> for U {
    fn execute_operation(
        &self,
        operation: <FFIFileIOWriteOutcomeListener as IsOutcomeListener>::Request,
        listener_rust_side: FFIFileIOWriteOutcomeListener,
    ) -> Result<(), FFISideError> {
        self.execute_file_io_write_request(operation, listener_rust_side.into())
    }
}
