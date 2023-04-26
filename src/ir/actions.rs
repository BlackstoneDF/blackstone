use crate::ir::values::*;
use strum::IntoEnumIterator;

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IRPlayerAction {
    GiveItems(Items, OptionalNumber),
    SetHotbarItems(Items),
    SetInventoryItems(Items),
    SetItemInSlot(OptionalItem, Number),
    SetEquipmentItem(OptionalItem, StringTag),
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
    SetArrowsStuck(OptionalNumber),
    SetBeeStingsStuck(OptionalNumber),
    SetVisualFire(),             // 1 tag
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
pub enum IREntityAction {
    Heal(OptionalNumber),
    SetCurrentHealth(Number),
    SetAbsorptionHealth(Number),
    SetMaximumHealth(Number), // 1 tag
    Damage(Number),
    SetFireTicks(Number),
    SetFreezeTicks(OptionalNumber), // 1 tag
    SetInvulnerabilityTicks(Number),
    GivePotionEffect(Potions), // 2 tags
    RemovePotionEffect(Potions),
    ClearPotionEffects,
    SetAnimalAge(Number), // 1 tag
    SetFallDistance(Number),
    SetCreeperFuse(Number),
    SetCreeperExplosionPower(Number),
    SetPotionCloudRadius(Number, OptionalNumber),
    SetVillagerExperience(Number),
    SetWitherInvulnerabilityTicks(Number),
    SetHorseJumpStrength(Number),
    SetPickupDelay(Number),
    SetFishingWaitTime(Number),
    SetWardenAngerLevel(Text, Number),
    DisguiseAsMob(Item, OptionalText),
    DisguiseAsPlayer(Text, OptionalItem),
    DisguiseAsBlock(Item, OptionalText),
    Undisguide,
    SetGlowing(),             // 1 tag
    SetDyeColor(),            // 1 tag
    SetTropicalFishPattern(), // 3 tags
    SetRabbitType(),          // 1 tag
    SetCatType(),             // 1 tag
    SetMooshroomType(),       // 1 tag
    SetFoxType(),             // 1 tag
    SetParrotColor(),         // 1 tag
    SetHorsePattern(),        // 2 tags
    SetAxolotlPattern(),      // 1 tag
    SetLlamaColor(),          // 1 tag
    SetFrogType(),            // 1 tag
    SetVillagerBiome(),       // 1 tag
    SetSnowGolemPumpkin(),    // 1 tag
    SetEndermanHeldBlock(Item),
    SetMinecartBlock(Item, OptionalNumber),
    SetArmorStandParts(), // 2 tags
    SetBeeHasNectar(),    // 2 tags
    SetProjectileDisplayItem(Item),
    SetVisualFire(),           // 1 tag
    SendMobAnimation(),        // 1 tag
    SendMobAttackAnimation(),  // 1 tag
    SetArmorStandPose(Vector), // 1 tag
    SetPose(),                 // 1 tag
    SetFoxLeaping(),           // 1 tag
    SetArmsRaised(),           // 1 tag
    SetCatResting(),           // 1 tag
    SetGlowSquidDarkTicks(Number),
    Teleport(Location),                             // 1 tag
    LaunchUp(Number),                               // 1 tag
    LaunchForward(Number),                          // 2 tags
    LaunchTowardLocation(Location, OptionalNumber), // 2 tags
    SetGliding(),                                   // 1 tag
    SetGravity(),                                   // 1 tag
    RideEntity(OptionalText),
    AttachLead(Text),
    // Not a DF command but represents the second option for AttachLead
    AttachLeadToFence(Location),
    SetRotation(Number, Number),
    SetVelocity(Vector),         // 1 tag
    SetCustomName(OptionalText), // 1 tag
    SetNameVisible(),            // 1 tag
    SetNameColor(OptionalText),
    SetAI(),                // 1 tag
    SetSilenced(),          // 1 tag
    SetDeathDropsEnabled(), // 1 tag
    SetCollidable(),        // 1 tag
    SetInvulnerable(),      // 1 tag
    SetSitting(),           // 1 tag
    SetBaby(),              // 1 tag
    SetSize(Number),
    SetSheepSheared(),               // 1 tag
    SetWearingSaddle(),              // 1 tag
    SetCarryingChest(),              // 1 tag
    SetArmorStandSlotInteractions(), // 2 tags
    SetMarker(),                     // 1 tag
    SetAngry(),                      // 1 tag
    SetRearing(),                    // 1 tag
    SetRiptiding(),                  // 1 tag
    SetCreeperCharged(),             // 1 tag
    SetInvisible(),                  // 1 tag
    SetGoatScreaming(),              // 1 tag
    SetGoatHorns(),                  // 2 tags
    Tame(OptionalText),
    SetEndCrystalBeam(OptionalLocation),
    SetPandaGene(),          // 2 tags
    SetVillagerProfession(), // 1 tag
    SetProjectileShooter(OptionalText),
    SetPersistent(), // 1 tag
    SetInteractionSize(OptionalNumber, OptionalNumber),
    SetInteractionResponsive(), // 1 tag
    SetCelebrating(),           // 1 tag
    SetTarget(OptionalText),
    MoveToLocation(Location, OptionalNumber),
    Jump,
    RamTarget(Text),
    EatTarget(Text),
    EatGrass,
    IgniteCreeper,
    Explode,
    SetFoxSleeping(), // 1 tag
    SetDragonPhase(), // 1 tag
    SetShulkerBulletTarget(OptionalText),
    UseItem(),         // 2 tags
    SetAllayDancing(), // 1 tag
    SetDisplayViewRange(OptionalNumber),
    SetDisplayBillboard(), // 1 tag
    SetDisplayShadow(OptionalNumber, OptionalNumber),
    SetDisplayBrightness(OptionalNumber, OptionalNumber),
    SetDisplayInterpolation(OptionalNumber, OptionalNumber),
    SetDisplayCullingSize(OptionalNumber, OptionalNumber),
    SetTextDisplayText(Texts),
    SetTextDisplayLineWidth(OptionalNumber),
    SetTextDisplayTextOpacity(OptionalNumber),
    SetTextDisplayTextAlignment(), // 1 tag
    SetTextDisplayTextShadow(),    // 1 tag
    SetTextDisplaySeeThrough(),    // 1 tag
    SetTextDisplayBackground(OptionalText, OptionalNumber),
    SetDisplayGlowColor(OptionalText),
    SetItemDisplayItem(Item),
    SetItemDisplayModelType(), // 1 tag
    SetBlockDisplayBlock(Item, OptionalTexts),
    SetDisplayTransformationMatrix(Numbers),
    SetDisplayROtationFromEulerAngles(Number, Number, Number), // 1 tag
    SetDisplayRotationFromAxisAngle(Vector, Number),           // 1 tag
    SetDisplayTranslation(Vector),
    SetDisplayScale(Vector),
    Remove,
    SetEquipmentItem(OptionalItem),
    SetArmorItems(Items),
    LaunchProjectile(
        Item,
        OptionalLocation,
        OptionalText,
        OptionalNumber,
        OptionalNumber,
    ),
    ShearSheep,
    SetCustomTag(Text, OptionalNumber),
    GetCustomTag(Variable, Text),
    RemoveCustomTag(Text),
    SetEntityItem(Item),
    SetWardenDigging(), // 1 tag
}

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
