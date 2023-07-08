    , { signature = "", internalName = "", action = explode, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = explodeFire, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = addMotion, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = blink, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = breakBlock, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = placeBlock, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = colorize, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = createWater, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = destroyWater, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = ignite, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = extinguish, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = conjureBlock, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = conjureLight, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "", internalName = "", action = bonemeal, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    
    , { signature = "qqqqqwaeaeaeaeaea", internalName = "recharge", action = recharge, displayName = "Recharge Item", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qdqawwaww", internalName = "erase", action = erase, displayName = "Erase Item", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wqaqwd", internalName = "edify", action = edify, displayName = "Edify Sapling", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "adaa", internalName = "beep", action = beep, displayName = "Make Note", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "waqqqqq", internalName = "craft/cypher", action = craftArtifact Cypher, displayName = "Craft Cypher", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wwaqqqqqeaqeaeqqqeaeq", internalName = "craft/trinket", action = craftArtifact Trinket, displayName = "Craft Trinket", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wwaqqqqqeawqwqwqwqwqwwqqeadaeqqeqqeadaeqq", internalName = "craft/artifact", action = craftArtifact Artifact, displayName = "Craft Artifact", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqaqwawaw", internalName = "potion/weakness", action = potion, displayName = "White Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqawwawawd", internalName = "potion/levitation", action = potionFixedPotency, displayName = "Blue Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqaewawawe", internalName = "potion/wither", action = potion, displayName = "Black Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqadwawaww", internalName = "potion/poison", action = potion, displayName = "Red Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqadwawaw", internalName = "potion/slowness", action = potion, displayName = "Green Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "waeawae", internalName = "sentinel/create", action = sentinelCreate, displayName = "Summon Sentinel", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qdwdqdw", internalName = "sentinel/destroy", action = sentinelDestroy, displayName = "Banish Sentinel", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "waeawaede", internalName = "sentinel/get_pos", action = sentinelGetPos, displayName = "Locate Sentinel", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "waeawaedwa", internalName = "sentinel/wayfind", action = sentinelWayfind, displayName = "Wayfind Sentinel", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqwqqqqqaq", internalName = "akashic/read", action = akashicRead, displayName = "Akasha's Distillation", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "eeeweeeeede", internalName = "akashic/write", action = akashicWrite, displayName = "Akasha's Gambit", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "aqqqqq", internalName = "read", action = read, displayName = "Scribe's Reflection", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wawqwqwqwqwqw", internalName = "read/entity", action = readChronical, displayName = "Chronicler's Purification", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "deeeee", internalName = "write", action = write, displayName = "Scribe's Gambit", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wdwewewewewew", internalName = "write/entity", action = writeChronical, displayName = "Chronicler's Gambit", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "aqqqqqe", internalName = "readable", action = readable, displayName = "Auditor's Reflection", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wawqwqwqwqwqwew", internalName = "readable/entity", action = makeConstant (Boolean False), displayName = "Auditor's Purification", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "deeeeeq", internalName = "writable", action = writable, displayName = "Assessor's Reflection", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wdwewewewewewqw", internalName = "writable/entity", action = makeConstant (Boolean False), displayName = "Assessor's Purification", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "awaawa", internalName = "save_macro", action = saveMacro, displayName = "Save Macro", outputOptions = [], selectedOutput = Nothing, startDirection = Southeast }