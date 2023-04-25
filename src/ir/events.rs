use strum::IntoEnumIterator;
use strum_macros;

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum PlayerEvent {
    Join,
    Leave,
    Command,
    RightClick,
    LeftClick,
    RightClickEntity,
    RightClickPlayer,
    PlaceBlock,
    BreakBlock,
    SwapHands,
    ChangeSlot,
    Walk,
    Jump,
    Sneak,
    Unsneak,
    StartSprint,
    StopSprint,
    StartFlight,
    StopFlight,
    Riptide,
    Dismount,
    HorseJump,
    VehicleJump,
    ClickMenuSlot,
    ClickInventorySlot,
    PickUpItem,
    DropItem,
    ConsumeItem,
    BreakItem,
    CloseInventory,
    Fish,
    PlayerTakeDamage,
    DamagePlayer,
    DamageEntity,
    EntityDamagePlayer,
    Heal,
    ShootBow,
    ShootProjectile,
    ProjectileHit,
    ProjectileDamagePlayer,
    PotionCloudImbue,
    PlayerDeath,
    PlayerKillPlayer,
    PlayerKillMob,
    KilledByMob,
    Respawn,
}

impl PlayerEvent {
    fn parse_player_event(code: &str) -> Option<PlayerEvent> {
        if !code.starts_with("playerEvent.") {
            return None;
        }
        let code = code.trim_start_matches("playerEvent.");
        PlayerEvent::iter().find(|event| event.to_string().eq_ignore_ascii_case(code))
    }
}

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum EntityEvent {
    EntityDamageEntity,
    EntityKillEntity,
    EntityTakeDamage,
    ProjectileDamageEntity,
    ProjectileKillEntity,
    EntityDeath,
    VehicleTakeDamage,
    BlockFall,
    FallingBlockLands,
}

impl EntityEvent {
    fn parse_entity_event(code: &str) -> Option<EntityEvent> {
        if !code.starts_with("entityEvent.") {
            return None;
        }
        let code = code.trim_start_matches("playerEvent.");
        EntityEvent::iter().find(|event| event.to_string().eq_ignore_ascii_case(code))
    }
}
