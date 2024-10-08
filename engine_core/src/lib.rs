mod app;
mod query;
mod scene;
mod schedule;
mod system;
mod thread;

/*
 * Re-export of frosty_alloc for ease of use.
 * In general for ids:
 *      0-500       are reserved for internal use
 *      1000-10000  are reserved for 2d
 *      10000-20000 are reserved for 3d
 *      everything after is open for individual projects
 *      These are prone to changing as the engine grows
 */
pub use frosty_alloc as alloc;
