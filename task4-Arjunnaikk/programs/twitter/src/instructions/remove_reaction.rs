//-------------------------------------------------------------------------------
use crate::errors::TwitterError;
use crate::states::*;
///
/// TASK: Implement the remove reaction functionality for the Twitter program
///
/// Requirements:
/// - Verify that the tweet reaction exists and belongs to the reaction author
/// - Decrement the appropriate counter (likes or dislikes) on the tweet
/// - Close the tweet reaction account and return rent to reaction author
///
///-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;

    match ctx.accounts.tweet_reaction.reaction {
        ReactionType::Like => match tweet.likes.checked_sub(1) {
            Some(value) => {
                tweet.likes = value;
                Ok(())
            }
            None => err!(TwitterError::MinLikesReached),
        },
        ReactionType::Dislike => match tweet.dislikes.checked_sub(1) {
            Some(value) => {
                tweet.dislikes = value;
                Ok(())
            }
            None => err!(TwitterError::MinDislikesReached),
        },
    }
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
      mut,
      has_one = reaction_author,
      close = reaction_author
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
