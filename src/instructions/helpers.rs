use crate::Escrow;
use core::mem::size_of;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

impl Escrow {
    /// The expected size of the `Escrow` account data in bytes.
    ///
    /// This is calculated by summing the sizes of all fields in the `Escrow` struct:
    /// - `seed`: u64 (8 bytes)
    /// - `maker`: Pubkey (32 bytes)
    /// - `mint_a`: Pubkey (32 bytes)
    /// - `mint_b`: Pubkey (32 bytes)
    /// - `receive`: u64 (8 bytes)
    /// - `bump`: [u8; 1] (1 byte)
    /// Total: 113 bytes.
    ///
    /// This constant is used to validate that the account data passed to the program
    /// matches the expected layout structure.
    pub const LEN: usize = size_of::<u64>()
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + size_of::<u64>()
        + size_of::<[u8; 1]>();

    /// Loads a mutable reference to the `Escrow` struct from a raw byte slice.
    ///
    /// This function performs a "zero-copy" deserialization. Instead of copying
    /// the bytes into a new struct (which consumes compute units), it reinterprets
    /// the memory address of the byte slice as a pointer to the `Escrow` struct.
    ///
    /// # Arguments
    /// * `bytes` - The mutable raw account data from the `AccountInfo`.
    ///
    /// # Returns
    /// * `Ok(&mut Self)` - A mutable reference to the `Escrow` struct if successful.
    /// * `Err(ProgramError)` - `InvalidAccountData` if the byte slice length is incorrect.
    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        // Validate that the account data has exactly the expected size.
        // This prevents reading/writing outside allocated memory or processing corrupt data.
        if bytes.len() != Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        // SAFETY:
        // 1. We validated the length above, ensuring we have enough bytes.
        // 2. `Escrow` is `#[repr(C)]`, ensuring a predictable memory layout.
        // 3. We use `transmute` to cast the pointer:
        //    - `bytes.as_mut_ptr()` gives us a `*mut u8` (pointer to the first byte).
        //    - We cast it to `*mut Self` (pointer to an Escrow struct).
        //    - We dereference it (`*`) and borrow it mutably (`&mut`).
        // This is safe assuming the alignment is correct (which typically is for u8 arrays on Solana).
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    /// Loads an immutable reference to the `Escrow` struct from a raw byte slice.
    ///
    /// Similar to `load_mut`, but for read-only access.
    ///
    /// # Arguments
    /// * `bytes` - The immutable raw account data.
    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        // Validation: Ensure the data length matches the struct definition.
        if bytes.len() != Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        // SAFETY:
        // Casts the `*const u8` pointer to a `*const Self` pointer.
        // Returns an immutable reference.
        Ok(unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    }

    /// Sets the `seed` field.
    /// Used during initialization to store the bump/seed used for PDA derivation.
    #[inline(always)]
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    /// Sets the `maker` field.
    /// Stores the public key of the user who initialized the escrow.
    #[inline(always)]
    pub fn set_maker(&mut self, maker: Pubkey) {
        self.maker = maker;
    }

    /// Sets the `mint_a` field.
    /// Stores the mint address of the token being deposited into the escrow.
    #[inline(always)]
    pub fn set_mint_a(&mut self, mint_a: Pubkey) {
        self.mint_a = mint_a;
    }

    /// Sets the `mint_b` field.
    /// Stores the mint address of the token the maker wants to receive in return.
    #[inline(always)]
    pub fn set_mint_b(&mut self, mint_b: Pubkey) {
        self.mint_b = mint_b;
    }

    /// Sets the `receive` field.
    /// Stores the amount of Token B the maker expects to receive.
    #[inline(always)]
    pub fn set_receive(&mut self, receive: u64) {
        self.receive = receive;
    }

    /// Sets the `bump` seed.
    /// Stores the canonical bump seed found during PDA derivation to validate
    /// future instructions signed by this PDA.
    #[inline(always)]
    pub fn set_bump(&mut self, bump: [u8; 1]) {
        self.bump = bump;
    }

    /// Helper to set all fields at once.
    ///
    /// This is a convenience method to initialize the entire struct in one call,
    /// typically used in the instruction that initializes the account (e.g., `Make` or `Deposit`).
    #[inline(always)]
    pub fn set_inner(
        &mut self,
        seed: u64,
        maker: Pubkey,
        mint_a: Pubkey,
        mint_b: Pubkey,
        receive: u64,
        bump: [u8; 1],
    ) {
        self.seed = seed;
        self.maker = maker;
        self.mint_a = mint_a;
        self.mint_b = mint_b;
        self.receive = receive;
        self.bump = bump;
    }
}
