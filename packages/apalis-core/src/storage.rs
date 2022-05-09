use chrono::Duration;
use futures::{future::BoxFuture, stream::BoxStream, Stream};
use serde::Serialize;

use crate::{
    error::StorageError,
    queue::Heartbeat,
    request::{JobRequest, JobState},
};

pub type StorageResult<I> = BoxFuture<'static, Result<I, StorageError>>;
pub type JobStream<T> = BoxStream<'static, Result<Option<JobRequest<T>>, StorageError>>;

/// Represents a [Storage] that can be passed to a [crate::builder::QueueBuilder]
pub trait Storage: Clone {
    type Output: Serialize;

    /// Pushes a job to a storage
    ///
    /// TODO: return id
    fn push(&mut self, job: Self::Output) -> StorageResult<()>;

    /// Get the stream of jobs
    fn consume(&mut self) -> JobStream<Self::Output>;

    fn len(&self) -> StorageResult<i64> {
        let fut = async { Ok(0) };
        Box::pin(fut)
    }

    fn ack(&mut self, job_id: String) -> StorageResult<()> {
        let fut = async { Ok(()) };
        Box::pin(fut)
    }

    fn retry(&mut self, job_id: String) -> StorageResult<()> {
        let fut = async { Ok(()) };
        Box::pin(fut)
    }

    fn heartbeat(&mut self, beat: Heartbeat) -> StorageResult<bool> {
        let fut = async { Ok(true) };
        Box::pin(fut)
    }

    fn kill(&mut self, job_id: String) -> StorageResult<()> {
        let fut = async { Ok(()) };
        Box::pin(fut)
    }

    fn reschedule(&mut self, job_id: String, wait: Duration) -> StorageResult<()> {
        let fut = async { Ok(()) };
        Box::pin(fut)
    }
}

pub trait StorageJobExt<Output>: Storage<Output = Output> {
    fn find_by_id(&mut self, job_id: String) -> StorageResult<Output>;
    fn list_by_page(&mut self, status: JobState, page: i32) -> StorageResult<Vec<Output>>;
}
