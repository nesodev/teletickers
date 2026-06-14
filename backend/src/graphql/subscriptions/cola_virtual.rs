use async_graphql::*;
use futures_util::Stream;
use futures_util::StreamExt;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

#[derive(Default)]
pub struct ColaVirtualSubscription;

#[derive(SimpleObject, Clone)]
pub struct ColaStatus {
    pub position: i32,
    pub estimated_wait_time: i32,
    pub active: bool,
}

#[Subscription]
impl ColaVirtualSubscription {
    async fn cola_status(&self, evento_id: String) -> Result<Pin<Box<dyn Stream<Item = ColaStatus> + Send>>> {
        let _evento_uuid = evento_id.parse::<uuid::Uuid>().map_err(|_| Error::new("Invalid evento_id"))?;

        let stream = IntervalStream::new(interval(Duration::from_secs(2)))
            .enumerate()
            .map(|(index, _)| {
                let position = if index < 10 { 10 - index as i32 } else { 0 };
                ColaStatus {
                    position,
                    estimated_wait_time: position * 30,
                    active: position > 0,
                }
            })
            .take(15);

        Ok(Box::pin(stream))
    }
}
