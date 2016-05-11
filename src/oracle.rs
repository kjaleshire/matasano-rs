use rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use std::marker::PhantomData;

use serialize::base64::FromBase64;

use aes;
use analyzer::Mode;
use utility::error::MatasanoError;

pub struct Oracle<'a, T: 'a> {
    pub append_str: Option<&'a T>,
    pub block_size: usize,
    pub last_key: Option<Vec<u8>>,
    pub last_mode: Mode,
    pub rng: rand::ThreadRng,
    _marker: PhantomData<&'a T>,
}

impl<'b, str> Oracle<'b, str> {
    pub fn new() -> Self {
        Oracle{
            append_str: None,
            block_size: 16,
            last_key: None,
            last_mode: Mode::None,
            rng: rand::thread_rng(),
            _marker: PhantomData{},
        }
    }

    pub fn new_with_append_str(append_str: &'b str) -> Self {
        Oracle{
            append_str: Some(append_str),
            block_size: 16,
            last_key: None,
            last_mode: Mode::None,
            rng: rand::thread_rng(),
            _marker: PhantomData{},
        }
    }

    pub fn generate_random_aes_key(&mut self) -> Vec<u8> {
        (0..self.block_size).map(|_| self.rng.gen()).collect()
    }

    pub fn randomly_mangled_encrypted_text(&mut self) -> Vec<u8> {
        let text_size = 3 * self.block_size;
        let random_byte: u8 = self.rng.gen();
        let prefix_size = Range::new(5, 11).ind_sample(&mut self.rng);
        let suffix_size = Range::new(5, 11).ind_sample(&mut self.rng);
        let vec_size = aes::padded_len(prefix_size + text_size + suffix_size, self.block_size);

        let mut mangled_text = Vec::with_capacity(vec_size);

        for _ in 0..prefix_size {
            mangled_text.push(self.rng.gen());
        }

        for _ in 0..text_size {
            mangled_text.push(random_byte);
        }

        for _ in 0..suffix_size {
            mangled_text.push(self.rng.gen());
        }

        let _ = aes::pkcs_pad_vec(&mut mangled_text, self.block_size);

        match self.rng.gen() {
            true => {
                self.last_mode = Mode::Cbc;
                let iv = vec![0; self.block_size];
                aes::encrypt_cbc_128_text(&mangled_text, &self.generate_random_aes_key(), &iv)
            },
            false => {
                self.last_mode = Mode::Ecb;
                aes::encrypt_ecb_128_text(&mangled_text, &self.generate_random_aes_key())
            }
        }
    }

    pub fn randomly_append_and_encrypt_text<'a>(&mut self, plain_text: &'a [u8]) -> Result<Vec<u8>, MatasanoError> {
        let append_vec = match self.append_str {
            Some(&thing) => thing.from_base64()?,
            None => return Err(MatasanoError::Other("Must set the append string before using this method"))
        };

        let vec_size = aes::padded_len(plain_text.len() + append_vec.len(), self.block_size);

        let mut mangled_text = Vec::with_capacity(vec_size);

        mangled_text.extend_from_slice(&plain_text);
        mangled_text.extend_from_slice(&append_vec);
        let _ = aes::pkcs_pad_vec(&mut mangled_text, self.block_size);

        self.last_mode = Mode::Ecb;

        match self.last_key {
            Some(ref key) => {
                Ok(aes::encrypt_ecb_128_text(&mangled_text, key))
            },
            None => {
                let key = self.generate_random_aes_key();
                let encoded_vec = aes::encrypt_ecb_128_text(&mangled_text, &key);
                self.last_key = Some(key);
                Ok(encoded_vec)
            }
        }
    }
}