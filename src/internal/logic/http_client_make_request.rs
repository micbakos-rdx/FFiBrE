use crate::prelude::*;

impl HTTPClient {
    pub(crate) async fn make_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse, NetworkError> {
        let (response_sender, response_receiver) = channel();
        let sender_wrapper = OneshotSenderWrapper::new(response_sender);
        self.request_sender
            .send(request, Arc::new(sender_wrapper))
            .unwrap();

        response_receiver
            .await
            .map_err(|_| RustSideError::FailedToReceiveResponseFromSwift)
            .map_err(|e| e.into())
            .and_then(|r| {
                let res: Result<NetworkResponse, SwiftSideError> = r.into();
                res.map_err(|e| e.into())
            })
    }
}