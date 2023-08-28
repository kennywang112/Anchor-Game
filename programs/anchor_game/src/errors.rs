use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("InvalidNFTOwner")]
    InvalidNFTOwner,
    #[msg("InvalidNFTAccountMint")]
    InvalidNFTAccountMint,
    #[msg("NFTAccountEmpty")]
    NFTAccountEmpty,
    #[msg("InvalidNFTMintSupply")]
    InvalidNFTMintSupply,
    #[msg("CollectionNotVerified")]
    CollectionNotVerified,
    #[msg("CollectionNotSame")]
    CollectionNotSame
}