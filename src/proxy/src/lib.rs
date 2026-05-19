/// AUXILIARY FUNCTIONS FOR DEVELOPMENT


pub struct ThreadPool;
 

impl ThreadPool {
    /// creates a new instance of threadPool
    /// it's a contained [`thread array`] that contains a limited amount of threads
    /// it works functionally like a queue, take a bit of information
    /// execute over it
    ///
    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static,
        {
        }
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        where
            F: FnOnce() -> T,
            F: Send + 'static,
            T: Send + 'static,
        {
        }
}
