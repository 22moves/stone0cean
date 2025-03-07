// First, let's update the StoneState enum to include activation levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum StoneState {
    Created,           // Created, but not activated
    ActivatedLevel1,   // Activated from distance (5km)
    ActivatedLevel2,   // Activated from medium range (1km)
    ActivatedLevel3,   // Activated from close range (300m)
    Locked,            // Temporarily locked (for updates)
}

// Next, let's modify the activation function to handle different levels

#[derive(Accounts)]
pub struct ActivateStone<'info> {
    // Keep the same account structure
    // ...
}

// Add a new parameter to the function for activation level
pub fn activate_stone(
    ctx: Context<ActivateStone>,
    review: Review,
    geo_proof: GeoProof,
    activation_level: u8,  // New parameter: 1, 2, or 3
) -> Result<()> {
    let stone = &mut ctx.accounts.stone;
    
    // Verify the stone is not already activated
    require!(
        stone.state == StoneState::Created ||
        (stone.state == StoneState::ActivatedLevel1 && activation_level > 1) ||
        (stone.state == StoneState::ActivatedLevel2 && activation_level > 2),
        ErrorCode::CannotDowngradeActivation
    );
    
    // Verify geo proximity based on activation level
    match activation_level {
        1 => verify_geo_proximity(&stone.location, &geo_proof, 5000)?,  // 5km
        2 => verify_geo_proximity(&stone.location, &geo_proof, 1000)?,  // 1km
        3 => verify_geo_proximity(&stone.location, &geo_proof, 300)?,   // 300m
        _ => return err!(ErrorCode::InvalidActivationLevel),
    }
    
    // Calculate activation cost based on proximity
    // Closer activation = higher rewards but also higher cost
    let activation_cost_multiplier = match activation_level {
        1 => 80,  // 80% of standard cost for level 1
        2 => 100, // 100% of standard cost for level 2
        3 => 120, // 120% of standard cost for level 3
        _ => 100,
    };
    
    let final_activation_cost = (stone.activation_cost * activation_cost_multiplier as u64) / 100;
    
    // Process payment with adjusted cost
    if final_activation_cost > 0 {
        // Payment distribution logic
        // ...
    }
    
    // Update stone state based on activation level
    stone.state = match activation_level {
        1 => StoneState::ActivatedLevel1,
        2 => StoneState::ActivatedLevel2,
        3 => StoneState::ActivatedLevel3,
        _ => stone.state, // shouldn't happen due to earlier validation
    };
    
    // Set initial moss level based on activation level
    let base_moss_level = match stone.stone_type {
        StoneType::Genesis => 20,
        StoneType::Astral => 10,
    };
    
    // Higher activation level = more initial moss
    let level_bonus = (activation_level - 1) * 5;
    stone.moss_level = base_moss_level + level_bonus;
    
    // Add review and update other stone properties
    // ...
    
    // Emit event with activation level
    emit!(StoneActivatedEvent {
        stone_id: stone.id,
        activator: ctx.accounts.activator.key(),
        timestamp: Clock::get()?.unix_timestamp,
        activation_level,
    });
    
    Ok(())
}

// Add helper function to verify proximity
fn verify_geo_proximity(location: &GeoData, proof: &GeoProof, max_distance_meters: u32) -> Result<()> {
    // In a real implementation, we would extract the user's reported position
    // from the geo_proof and calculate the actual distance
    
    // Example with hypothetical geo_proof that includes user coordinates
    let user_latitude = extract_latitude_from_proof(proof);
    let user_longitude = extract_longitude_from_proof(proof);
    
    let distance = calculate_distance(
        location.latitude, location.longitude,
        user_latitude, user_longitude
    );
    
    require!(
        distance <= max_distance_meters as f64,
        ErrorCode::OutsideActivationRange
    );
    
    // Also perform standard proof verification
    verify_geo_proof(location, proof, proof.oracle)?;
    
    Ok(())
}

// Helper function to extract latitude from proof (would be implemented in a real contract)
fn extract_latitude_from_proof(proof: &GeoProof) -> f64 {
    // For example purposes, we're returning a placeholder
    // In reality, this would decode the lat/long from the signed proof data
    0.0
}

// Helper function to extract longitude from proof (would be implemented in a real contract)
fn extract_longitude_from_proof(proof: &GeoProof) -> f64 {
    // For example purposes, we're returning a placeholder
    // In reality, this would decode the lat/long from the signed proof data
    0.0
}

// Update the StoneActivatedEvent to include activation level
#[event]
pub struct StoneActivatedEvent {
    pub stone_id: u64,
    pub activator: Pubkey,
    pub timestamp: i64,
    pub activation_level: u8,
}

// Add new error codes
#[error_code]
pub enum ErrorCode {
    // ... existing errors
    
    #[msg("Cannot downgrade activation level")]
    CannotDowngradeActivation,
    
    #[msg("Invalid activation level")]
    InvalidActivationLevel,
    
    #[msg("Outside activation range")]
    OutsideActivationRange,
}

// We also need to update the grow_moss function to consider activation levels
pub fn grow_moss(ctx: Context<GrowMoss>) -> Result<()> {
    let stone = &mut ctx.accounts.stone;
    let current_time = ctx.accounts.clock.unix_timestamp;
    
    // Check that the stone is activated at any level
    require!(
        stone.state == StoneState::ActivatedLevel1 || 
        stone.state == StoneState::ActivatedLevel2 || 
        stone.state == StoneState::ActivatedLevel3,
        ErrorCode::InvalidState
    );
    
    // Calculate time-based growth factor
    // ... [existing code]
    
    // Apply activation level bonus to growth
    let activation_level_bonus = match stone.state {
        StoneState::ActivatedLevel1 => 1,
        StoneState::ActivatedLevel2 => 2,
        StoneState::ActivatedLevel3 => 3,
        _ => 0,
    };
    
    let growth = base_growth + spore_bonus + env_bonus + activation_level_bonus;
    stone.moss_level = stone.moss_level.saturating_add(growth).min(100);
    
    // Calculate rewards with level bonus
    let level_reward_multiplier = match stone.state {
        StoneState::ActivatedLevel1 => 100,
        StoneState::ActivatedLevel2 => 125,
        StoneState::ActivatedLevel3 => 150,
        _ => 100,
    };
    
    let base_tokens = calculate_moss_rewards(stone);
    let tokens_to_mint = (base_tokens * level_reward_multiplier as u64) / 100;
    
    // Mint tokens and update last harvest time
    // ... [existing code]
    
    Ok(())
}
