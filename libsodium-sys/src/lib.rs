#![feature(maybe_uninit_slice)]

use std::mem::MaybeUninit;

/// ffi should be a separate crate because you're more likely to make
/// breaking changes to the wrapping api and shouldn't do them to the *-sys crate
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// The only way to get a Sodium is by calling new().
// External users cannot construct or destruct a sodium. Same effect as giving the type a non public field
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub struct Sodium;

impl Sodium {
    /// Assures that sodium_init was called without an error
    pub fn new() -> Result<Self, ()> {
        if unsafe { ffi::sodium_init() } < 0 {
            Err(())
        } else {
            Ok(Self)
        }
    }

    /// https://libsodium.gitbook.io/doc/hashing/generic_hashing
    pub fn crypto_generichash<'a>(
        self,
        input: &[u8],
        key: Option<&[u8]>,
        out: &'a mut [MaybeUninit<u8>],
    ) -> Result<&'a mut [u8], ()> {
        assert!(out.len() >= usize::try_from(ffi::crypto_generichash_BYTES_MIN).unwrap());
        assert!(out.len() <= usize::try_from(ffi::crypto_generichash_BYTES_MAX).unwrap());

        let (key, keylen) = if let Some(key) = key {
            assert!(out.len() >= usize::try_from(ffi::crypto_generichash_KEYBYTES_MIN).unwrap());
            assert!(out.len() <= usize::try_from(ffi::crypto_generichash_KEYBYTES_MAX).unwrap());
            (key.as_ptr(), key.len())
        } else {
            (std::ptr::null(), 0)
        };

        // SAFETY: We've checked the reqs of the function MIN/MAX and
        // the presence of self mens that sodium_init has been called
        let res = unsafe {
            ffi::crypto_generichash(
                MaybeUninit::slice_as_mut_ptr(out),
                out.len() as u64,
                input.as_ptr(),
                input.len() as u64,
                key,
                keylen as u64,
            )
        };
        if res < 0 {
            return Err(());
        } else {
            // SAFETY: crypto_generichash writes to (and thus initializes) all the bytes of `out`.
            return Ok(unsafe { MaybeUninit::slice_assume_init_mut(out) });
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Sodium::new().unwrap();
    }
    #[test]
    fn it_hashes() {
        let s = Sodium::new().unwrap();
        // check against echo -n "Arbitrary data to hash" | b2sum -l256
        let input = b"Arbitrary data to hash";
        let mut out = [MaybeUninit::uninit(); ffi::crypto_generichash_BYTES as usize];
        let res = s.crypto_generichash(input, None, &mut out).unwrap();

        println!("{}", hex::encode(res));
    }
}
