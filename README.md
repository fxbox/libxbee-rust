# libxbee-rust
A wrapper for libxbee

The xbee.rs file has been originally generated by rust-bindgen.

Manually added the `libc`, `va_list` crates and use of `time_t, timespec, va_list::VaList, std::fs::File`

According to [this](https://github.com/thepowersgang/va_list-rs/issues/3) the modification made to use VaList has good chances to work. The changes for File might not.
