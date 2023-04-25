use crate::ir::values::*;
use strum::IntoEnumIterator;

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IRPlayerAction {
    GiveItems(Items, OptionalNumber),
    SetHotbarItems(Items),
    SetInventoryItems(Items),
    SetItemInSlot(OptionalItem, Number),
    SetEquipmentItem(OptionalItem), // tag
    SetArmorItems(Items),
    ReplaceItems(OptionalItems, Item, OptionalNumber),
    RemoveItems(Items),
    ClearItems(Items),
    ClearInventory(), // 2 tags
    SetCursorItem(OptionalItem),
    SaveCurrentInventory,
    LoadSavedInventory,
    SetItemCooldown(Item, Number),
    SendMessage(Texts), // 2 tags
    SendMessageSequence(Texts, OptionalNumber),
    SendHoverMessage(Text, Text),
    ShowTitleText(
        Text,
        OptionalText,
        OptionalNumber,
        OptionalNumber,
        OptionalNumber,
    ),
    ShowActionBarText(Texts), // 1 tag
    OpenBook(Item),
    SetBossBar(OptionalText, OptionalNumber, OptionalNumber, OptionalNumber), // 3 tags
    RemoveBossBar(OptionalNumber),
    SendAdvancement(Text, Item),         // 1 tag
    SetPlayerListInfo(OptionalTexts),    // 1 tag
    PlaySound(Sounds, OptionalLocation), // 1 tag
    StopSounds(OptionalSounds),          // 1 tag
    PlaySoundFromEntity(Sounds, Text),   // 1 tag
    ShowInventoryMenu(OptionalItems),
    ExpandInventoryMenu(OptionalItems),
    SetInventoryMenuItem(Number, OptionalItem),
    SetInventoryMenuName(Text),
    AddInventoryMenuRow(OptionalItems),     // 1 tag
    RemoveInventoryMenuRow(OptionalNumber), // 1 tag
    CloseInventory,
    OpenContainerInventory(Location),
    SetScoreboardObjectiveName(Text),
    SetSidebarVisible(), // 1 tag
    SetScoreboardScore(Text, OptionalNumber),
    RemoveScoreboardScore(Text),
    ClearScoreboard,
    Damage(Number, OptionalText),
    Heal(OptionalNumber),
    SetCurrentHealth(Number),
    SetMaximumHealth(Number), // 1 tag
    SetAbsorptionHealth(Number),
    SetFoodLevel(Number),
    SetSaturationLevel(Number),
    GiveExperience(Number),    // 1 tag
    SetExperience(Number),     // 1 tag
    GivePotionEffect(Potions), // 3 tags
    RemovePotionEffect(Potions),
    ClearPotionEffects,
    SetHotbarSlot(Number),
    SetBaseAttackSpeed(OptionalNumber),
    SetFireTicks(Number),
    SetFreezeTicks(Number),
    SetRemainingAir(Number),
    SetInvulnerabilityTicks(Number),
    SetFallDistance(Number),
    SetMovementSpeed(Number), // 1 tag
    SetSurvivalMode,
    SetAdventureMode,
    SetCreativeMode,
    SetSpectatorMode,
    SetAllowFlight(),       // 1 tag
    SetPVPAllowed(),        // 1 tag
    SetDeathDropsEnabled(), // 1 tag
    SetInventoryKept(),     // 1 tag
    SetCollidable(),        // 1 tag
    AllowPlaceBreakBlocks(OptionalItems),
    DisallowPlaceBreakBlocks(OptionalItems),
    SetInstantRespawn(),                            // 1 tag
    SetReducedDebugInfo(),                          // 1 tag
    Teleport(Location),                             // 2 tags
    LaunchUp(Number),                               // 1 tag
    LaunchForward(Number),                          // 2 tags
    LaunchTowardLocation(Location, OptionalNumber), // 2 tags
    RideEntity(OptionalText),
    SetFlying(),  // 1 tag
    SetGliding(), // 1 tag
    BoostElytra(Item),
    SetRotation(Number, Number),
    FaceLocation(Location),
    SetVelocity(Vector), // 1 tag
    SpectateTarget(OptionalVector),
    SetSpawnPoint(OptionalLocation),
    LaunchProjectile(
        Item,
        OptionalLocation,
        OptionalText,
        OptionalNumber,
        OptionalNumber,
    ),
    SetPlayerTime(OptionalNumber),
    SetPlayerWeather(), // 1 tag
    SetCompassTarget(Location),
    DisplayBlock(Item, Location, OptionalLocation, OptionalTexts),
    DisplayBlockFracture(Locations, OptionalNumber),
    DisplayBlockOpenedState(Location),        // 1 tag
    DisplayGatewayBeam(Location),             // 1 tag
    DisplaySignText(Location, OptionalTexts), // 2 tags
    DisplayHologram(Location, OptionalText),
    SetFogDistance(OptionalNumber),
    SetWorldBorder(Location, Number, OptionalNumber),
    ShiftWorldBorder(Number, OptionalNumber),
    RemoveWorldBorder,
    DisplayPickUpAnimation(Text, Text),
    SetEntityHidden(Text), // 1 tag
    DisplayParticleEffect(Particles, Location),
    DisplayParticleLine(Particle, Location, Location, OptionalNumber),
    DisplayAnimatedParticleLine(Particle, Location, OptionalNumber, OptionalNumber),
    DisplayParticleCircle(Particle, Location, OptionalNumber),
    DisplayAnimatedParticleCircle(Particle, Location, OptionalNumber, OptionalNumber),
    DisplayParticleCuboid(Particle, Location, Location, OptionalNumber), // 1 tag
    DisplayAnimatedParticleCuboi(Particle, Location, Location, OptionalNumber, OptionalNumber), // 1 tag
    DisplayParticleSpiral(
        Particle,
        Location,
        OptionalNumber,
        OptionalNumber,
        OptionalNumber,
        OptionalNumber,
    ),
    DisplayAnimatedParticleSpiral(
        Particle,
        Location,
        OptionalNumber,
        OptionalNumber,
        OptionalNumber,
        OptionalNumber,
        OptionalNumber,
    ),
    DisplayParticleSphere(Particle, Location, OptionalNumber),
    DisplayParticleRay(Particle, Location, Vector, OptionalNumber),
    DisplayLightningBolt(Location),
    DisplayVibrationEffect(Location, Location, OptionalNumber),
    DisguiseAsMob(Item, OptionalText),
    DisguiseAsPlayer(Text, OptionalItem),
    DisguiseAsBlock(Item, OptionalText),
    SetOwnDisguiseVisibility(), // 1 tag
    Undisguide,
    SetChatTag(OptionalTexts),
    SetChatColor(OptionalText),
    SetNameColor(OptionalText),
    SetArrowsStucks(OptionalNumber),
    SetBeeStingsStuck(OptionalNumber),
    SetVisualVire(),             // 1 tag
    SendPlayerAttackAnimation(), // 1 tag
    SendPlayerHurtAnimation(OptionalLocation),
    SendWakeUpAnimation,
    SetStatus(OptionalText),
    SetSkin(OptionalItem),
    RollBackBlockChanges(Number),
    Kick,
}

#[allow(dead_code)]
impl IRPlayerAction {
    fn parse_player_action(code: &str) -> Option<IRPlayerAction> {
        if !code.starts_with("playerAction.") {
            return None;
        }
        let code = code.trim_start_matches("playerAction.");
        IRPlayerAction::iter().find(|event| event.to_string().eq_ignore_ascii_case(code))
    }
}

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IRGameAction {}

#[allow(dead_code)]
impl IRGameAction {
    fn parse_game_action(code: &str) -> Option<IRGameAction> {
        if !code.starts_with("gameAction.") {
            return None;
        }
        let code = code.trim_start_matches("gameAction.");
        IRGameAction::iter().find(|event| event.to_string().eq_ignore_ascii_case(code))
    }
}

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IREntityAction {}

#[allow(dead_code)]
impl IREntityAction {
    fn parse_entity_action(code: &str) -> Option<IREntityAction> {
        if !code.starts_with("entityAction.") {
            return None;
        }
        let code = code.trim_start_matches("entityAction.");
        IREntityAction::iter().find(|event| event.to_string().eq_ignore_ascii_case(code))
    }
}
