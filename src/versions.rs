/// enum EUnrealEngineObjectUE4Version from Engine/Source/Runtime/Core/Public/UObject/ObjectVersion.h
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
pub enum UnrealEngineObjectUE4Version {
    VER_UE4_OLDEST_LOADABLE_PACKAGE = 214,
    VER_UE4_BLUEPRINT_VARS_NOT_READ_ONLY = 215,
    VER_UE4_STATIC_MESH_STORE_NAV_COLLISION = 216,
    VER_UE4_ATMOSPHERIC_FOG_DECAY_NAME_CHANGE = 217,
    VER_UE4_SCENECOMP_TRANSLATION_TO_LOCATION = 218,
    VER_UE4_MATERIAL_ATTRIBUTES_REORDERING = 219,
    VER_UE4_COLLISION_PROFILE_SETTING = 220,
    VER_UE4_BLUEPRINT_SKEL_TEMPORARY_TRANSIENT = 221,
    VER_UE4_BLUEPRINT_SKEL_SERIALIZED_AGAIN = 222,
    VER_UE4_BLUEPRINT_SETS_REPLICATION = 223,
    VER_UE4_WORLD_LEVEL_INFO = 224,
    VER_UE4_AFTER_CAPSULE_HALF_HEIGHT_CHANGE = 225,
    VER_UE4_ADDED_NAMESPACE_AND_KEY_DATA_TO_FTEXT = 226,
    VER_UE4_ATTENUATION_SHAPES = 227,
    VER_UE4_LIGHTCOMPONENT_USE_IES_TEXTURE_MULTIPLIER_ON_NON_IES_BRIGHTNESS = 228,
    VER_UE4_REMOVE_INPUT_COMPONENTS_FROM_BLUEPRINTS = 229,
    VER_UE4_VARK2NODE_USE_MEMBERREFSTRUCT = 230,
    VER_UE4_REFACTOR_MATERIAL_EXPRESSION_SCENECOLOR_AND_SCENEDEPTH_INPUTS = 231,
    VER_UE4_SPLINE_MESH_ORIENTATION = 232,
    VER_UE4_REVERB_EFFECT_ASSET_TYPE = 233,
    VER_UE4_MAX_TEXCOORD_INCREASED = 234,
    VER_UE4_SPEEDTREE_STATICMESH = 235,
    VER_UE4_LANDSCAPE_COMPONENT_LAZY_REFERENCES = 236,
    VER_UE4_SWITCH_CALL_NODE_TO_USE_MEMBER_REFERENCE = 237,
    VER_UE4_ADDED_SKELETON_ARCHIVER_REMOVAL = 238,
    VER_UE4_ADDED_SKELETON_ARCHIVER_REMOVAL_SECOND_TIME = 239,
    VER_UE4_BLUEPRINT_SKEL_CLASS_TRANSIENT_AGAIN = 240,
    VER_UE4_ADD_COOKED_TO_UCLASS = 241,
    VER_UE4_DEPRECATED_STATIC_MESH_THUMBNAIL_PROPERTIES_REMOVED = 242,
    VER_UE4_COLLECTIONS_IN_SHADERMAPID = 243,
    VER_UE4_REFACTOR_MOVEMENT_COMPONENT_HIERARCHY = 244,
    VER_UE4_FIX_TERRAIN_LAYER_SWITCH_ORDER = 245,
    VER_UE4_ALL_PROPS_TO_CONSTRAINTINSTANCE = 246,
    VER_UE4_LOW_QUALITY_DIRECTIONAL_LIGHTMAPS = 247,
    VER_UE4_ADDED_NOISE_EMITTER_COMPONENT = 248,
    VER_UE4_ADD_TEXT_COMPONENT_VERTICAL_ALIGNMENT = 249,
    VER_UE4_ADDED_FBX_ASSET_IMPORT_DATA = 250,
    VER_UE4_REMOVE_LEVELBODYSETUP = 251,
    VER_UE4_REFACTOR_CHARACTER_CROUCH = 252,
    VER_UE4_SMALLER_DEBUG_MATERIALSHADER_UNIFORM_EXPRESSIONS = 253,
    VER_UE4_APEX_CLOTH = 254,
    VER_UE4_SAVE_COLLISIONRESPONSE_PER_CHANNEL = 255,
    VER_UE4_ADDED_LANDSCAPE_SPLINE_EDITOR_MESH = 256,
    VER_UE4_CHANGED_MATERIAL_REFACTION_TYPE = 257,
    VER_UE4_REFACTOR_PROJECTILE_MOVEMENT = 258,
    VER_UE4_REMOVE_PHYSICALMATERIALPROPERTY = 259,
    VER_UE4_PURGED_FMATERIAL_COMPILE_OUTPUTS = 260,
    VER_UE4_ADD_COOKED_TO_LANDSCAPE = 261,
    VER_UE4_CONSUME_INPUT_PER_BIND = 262,
    VER_UE4_SOUND_CLASS_GRAPH_EDITOR = 263,
    VER_UE4_FIXUP_TERRAIN_LAYER_NODES = 264,
    VER_UE4_RETROFIT_CLAMP_EXPRESSIONS_SWAP = 265,
    VER_UE4_REMOVE_LIGHT_MOBILITY_CLASSES = 266,
    VER_UE4_REFACTOR_PHYSICS_BLENDING = 267,
    VER_UE4_WORLD_LEVEL_INFO_UPDATED = 268,
    VER_UE4_STATIC_SKELETAL_MESH_SERIALIZATION_FIX = 269,
    VER_UE4_REMOVE_STATICMESH_MOBILITY_CLASSES = 270,
    VER_UE4_REFACTOR_PHYSICS_TRANSFORMS = 271,
    VER_UE4_REMOVE_ZERO_TRIANGLE_SECTIONS = 272,
    VER_UE4_CHARACTER_MOVEMENT_DECELERATION = 273,
    VER_UE4_CAMERA_ACTOR_USING_CAMERA_COMPONENT = 274,
    VER_UE4_CHARACTER_MOVEMENT_DEPRECATE_PITCH_ROLL = 275,
    VER_UE4_REBUILD_TEXTURE_STREAMING_DATA_ON_LOAD = 276,
    VER_UE4_SUPPORT_32BIT_STATIC_MESH_INDICES = 277,
    VER_UE4_ADDED_CHUNKID_TO_ASSETDATA_AND_UPACKAGE = 278,
    VER_UE4_CHARACTER_DEFAULT_MOVEMENT_BINDINGS = 279,
    VER_UE4_APEX_CLOTH_LOD = 280,
    VER_UE4_ATMOSPHERIC_FOG_CACHE_DATA = 281,
    VAR_UE4_ARRAY_PROPERTY_INNER_TAGS = 282,
    VER_UE4_KEEP_SKEL_MESH_INDEX_DATA = 283,
    VER_UE4_BODYSETUP_COLLISION_CONVERSION = 284,
    VER_UE4_REFLECTION_CAPTURE_COOKING = 285,
    VER_UE4_REMOVE_DYNAMIC_VOLUME_CLASSES = 286,
    VER_UE4_STORE_HASCOOKEDDATA_FOR_BODYSETUP = 287,
    VER_UE4_REFRACTION_BIAS_TO_REFRACTION_DEPTH_BIAS = 288,
    VER_UE4_REMOVE_SKELETALPHYSICSACTOR = 289,
    VER_UE4_PC_ROTATION_INPUT_REFACTOR = 290,
    VER_UE4_LANDSCAPE_PLATFORMDATA_COOKING = 291,
    VER_UE4_CREATEEXPORTS_CLASS_LINKING_FOR_BLUEPRINTS = 292,
    VER_UE4_REMOVE_NATIVE_COMPONENTS_FROM_BLUEPRINT_SCS = 293,
    VER_UE4_REMOVE_SINGLENODEINSTANCE = 294,
    VER_UE4_CHARACTER_BRAKING_REFACTOR = 295,
    VER_UE4_VOLUME_SAMPLE_LOW_QUALITY_SUPPORT = 296,
    VER_UE4_SPLIT_TOUCH_AND_CLICK_ENABLES = 297,
    VER_UE4_HEALTH_DEATH_REFACTOR = 298,
    VER_UE4_SOUND_NODE_ENVELOPER_CURVE_CHANGE = 299,
    VER_UE4_POINT_LIGHT_SOURCE_RADIUS = 300,
    VER_UE4_SCENE_CAPTURE_CAMERA_CHANGE = 301,
    VER_UE4_MOVE_SKELETALMESH_SHADOWCASTING = 302,
    VER_UE4_CHANGE_SETARRAY_BYTECODE = 303,
    VER_UE4_MATERIAL_INSTANCE_BASE_PROPERTY_OVERRIDES = 304,
    VER_UE4_COMBINED_LIGHTMAP_TEXTURES = 305,
    VER_UE4_BUMPED_MATERIAL_EXPORT_GUIDS = 306,
    VER_UE4_BLUEPRINT_INPUT_BINDING_OVERRIDES = 307,
    VER_UE4_FIXUP_BODYSETUP_INVALID_CONVEX_TRANSFORM = 308,
    VER_UE4_FIXUP_STIFFNESS_AND_DAMPING_SCALE = 309,
    VER_UE4_REFERENCE_SKELETON_REFACTOR = 310,
    VER_UE4_K2NODE_REFERENCEGUIDS = 311,
    VER_UE4_FIXUP_ROOTBONE_PARENT = 312,
    VER_UE4_TEXT_RENDER_COMPONENTS_WORLD_SPACE_SIZING = 313,
    VER_UE4_MATERIAL_INSTANCE_BASE_PROPERTY_OVERRIDES_PHASE_2 = 314,
    VER_UE4_CLASS_NOTPLACEABLE_ADDED = 315,
    VER_UE4_WORLD_LEVEL_INFO_LOD_LIST = 316,
    VER_UE4_CHARACTER_MOVEMENT_VARIABLE_RENAMING_1 = 317,
    VER_UE4_FSLATESOUND_CONVERSION = 318,
    VER_UE4_WORLD_LEVEL_INFO_ZORDER = 319,
    VER_UE4_PACKAGE_REQUIRES_LOCALIZATION_GATHER_FLAGGING = 320,
    VER_UE4_BP_ACTOR_VARIABLE_DEFAULT_PREVENTING = 321,
    VER_UE4_TEST_ANIMCOMP_CHANGE = 322,
    VER_UE4_EDITORONLY_BLUEPRINTS = 323,
    VER_UE4_EDGRAPHPINTYPE_SERIALIZATION = 324,
    VER_UE4_NO_MIRROR_BRUSH_MODEL_COLLISION = 325,
    VER_UE4_CHANGED_CHUNKID_TO_BE_AN_ARRAY_OF_CHUNKIDS = 326,
    VER_UE4_WORLD_NAMED_AFTER_PACKAGE = 327,
    VER_UE4_SKY_LIGHT_COMPONENT = 328,
    VER_UE4_WORLD_LAYER_ENABLE_DISTANCE_STREAMING = 329,
    VER_UE4_REMOVE_ZONES_FROM_MODEL = 330,
    VER_UE4_FIX_ANIMATIONBASEPOSE_SERIALIZATION = 331,
    VER_UE4_SUPPORT_8_BONE_INFLUENCES_SKELETAL_MESHES = 332,
    VER_UE4_ADD_OVERRIDE_GRAVITY_FLAG = 333,
    VER_UE4_SUPPORT_GPUSKINNING_8_BONE_INFLUENCES = 334,
    VER_UE4_ANIM_SUPPORT_NONUNIFORM_SCALE_ANIMATION = 335,
    VER_UE4_ENGINE_VERSION_OBJECT = 336,
    VER_UE4_PUBLIC_WORLDS = 337,
    VER_UE4_SKELETON_GUID_SERIALIZATION = 338,
    VER_UE4_CHARACTER_MOVEMENT_WALKABLE_FLOOR_REFACTOR = 339,
    VER_UE4_INVERSE_SQUARED_LIGHTS_DEFAULT = 340,
    VER_UE4_DISABLED_SCRIPT_LIMIT_BYTECODE = 341,
    VER_UE4_PRIVATE_REMOTE_ROLE = 342,
    VER_UE4_FOLIAGE_STATIC_MOBILITY = 343,
    VER_UE4_BUILD_SCALE_VECTOR = 344,
    VER_UE4_FOLIAGE_COLLISION = 345,
    VER_UE4_SKY_BENT_NORMAL = 346,
    VER_UE4_LANDSCAPE_COLLISION_DATA_COOKING = 347,
    VER_UE4_MORPHTARGET_CPU_TANGENTZDELTA_FORMATCHANGE = 348,
    VER_UE4_SOFT_CONSTRAINTS_USE_MASS = 349,
    VER_UE4_REFLECTION_DATA_IN_PACKAGES = 350,
    VER_UE4_FOLIAGE_MOVABLE_MOBILITY = 351,
    VER_UE4_UNDO_BREAK_MATERIALATTRIBUTES_CHANGE = 352,
    VER_UE4_ADD_CUSTOMPROFILENAME_CHANGE = 353,
    VER_UE4_FLIP_MATERIAL_COORDS = 354,
    VER_UE4_MEMBERREFERENCE_IN_PINTYPE = 355,
    VER_UE4_VEHICLES_UNIT_CHANGE = 356,
    VER_UE4_ANIMATION_REMOVE_NANS = 357,
    VER_UE4_SKELETON_ASSET_PROPERTY_TYPE_CHANGE = 358,
    VER_UE4_FIX_BLUEPRINT_VARIABLE_FLAGS = 359,
    VER_UE4_VEHICLES_UNIT_CHANGE2 = 360,
    VER_UE4_UCLASS_SERIALIZE_INTERFACES_AFTER_LINKING = 361,
    VER_UE4_STATIC_MESH_SCREEN_SIZE_LODS = 362,
    VER_UE4_FIX_MATERIAL_COORDS = 363,
    VER_UE4_SPEEDTREE_WIND_V7 = 364,
    VER_UE4_LOAD_FOR_EDITOR_GAME = 365,
    VER_UE4_SERIALIZE_RICH_CURVE_KEY = 366,
    VER_UE4_MOVE_LANDSCAPE_MICS_AND_TEXTURES_WITHIN_LEVEL = 367,
    VER_UE4_FTEXT_HISTORY = 368,
    VER_UE4_FIX_MATERIAL_COMMENTS = 369,
    VER_UE4_STORE_BONE_EXPORT_NAMES = 370,
    VER_UE4_MESH_EMITTER_INITIAL_ORIENTATION_DISTRIBUTION = 371,
    VER_UE4_DISALLOW_FOLIAGE_ON_BLUEPRINTS = 372,
    VER_UE4_FIXUP_MOTOR_UNITS = 373,
    VER_UE4_DEPRECATED_MOVEMENTCOMPONENT_MODIFIED_SPEEDS = 374,
    VER_UE4_RENAME_CANBECHARACTERBASE = 375,
    VER_UE4_GAMEPLAY_TAG_CONTAINER_TAG_TYPE_CHANGE = 376,
    VER_UE4_FOLIAGE_SETTINGS_TYPE = 377,
    VER_UE4_STATIC_SHADOW_DEPTH_MAPS = 378,
    VER_UE4_ADD_TRANSACTIONAL_TO_DATA_ASSETS = 379,
    VER_UE4_ADD_LB_WEIGHTBLEND = 380,
    VER_UE4_ADD_ROOTCOMPONENT_TO_FOLIAGEACTOR = 381,
    VER_UE4_FIX_MATERIAL_PROPERTY_OVERRIDE_SERIALIZE = 382,
    VER_UE4_ADD_LINEAR_COLOR_SAMPLER = 383,
    VER_UE4_ADD_STRING_ASSET_REFERENCES_MAP = 384,
    VER_UE4_BLUEPRINT_USE_SCS_ROOTCOMPONENT_SCALE = 385,
    VER_UE4_LEVEL_STREAMING_DRAW_COLOR_TYPE_CHANGE = 386,
    VER_UE4_CLEAR_NOTIFY_TRIGGERS = 387,
    VER_UE4_SKELETON_ADD_SMARTNAMES = 388,
    VER_UE4_ADDED_CURRENCY_CODE_TO_FTEXT = 389,
    VER_UE4_ENUM_CLASS_SUPPORT = 390,
    VER_UE4_FIXUP_WIDGET_ANIMATION_CLASS = 391,
    VER_UE4_SOUND_COMPRESSION_TYPE_ADDED = 392,
    VER_UE4_AUTO_WELDING = 393,
    VER_UE4_RENAME_CROUCHMOVESCHARACTERDOWN = 394,
    VER_UE4_LIGHTMAP_MESH_BUILD_SETTINGS = 395,
    VER_UE4_RENAME_SM3_TO_ES3_1 = 396,
    VER_UE4_DEPRECATE_UMG_STYLE_ASSETS = 397,
    VER_UE4_POST_DUPLICATE_NODE_GUID = 398,
    VER_UE4_RENAME_CAMERA_COMPONENT_VIEW_ROTATION = 399,
    VER_UE4_CASE_PRESERVING_FNAME = 400,
    VER_UE4_RENAME_CAMERA_COMPONENT_CONTROL_ROTATION = 401,
    VER_UE4_FIX_REFRACTION_INPUT_MASKING = 402,
    VER_UE4_GLOBAL_EMITTER_SPAWN_RATE_SCALE = 403,
    VER_UE4_CLEAN_DESTRUCTIBLE_SETTINGS = 404,
    VER_UE4_CHARACTER_MOVEMENT_UPPER_IMPACT_BEHAVIOR = 405,
    VER_UE4_BP_MATH_VECTOR_EQUALITY_USES_EPSILON = 406,
    VER_UE4_FOLIAGE_STATIC_LIGHTING_SUPPORT = 407,
    VER_UE4_SLATE_COMPOSITE_FONTS = 408,
    VER_UE4_REMOVE_SAVEGAMESUMMARY = 409,
    VER_UE4_REMOVE_SKELETALMESH_COMPONENT_BODYSETUP_SERIALIZATION = 410,
    VER_UE4_SLATE_BULK_FONT_DATA = 411,
    VER_UE4_ADD_PROJECTILE_FRICTION_BEHAVIOR = 412,
    VER_UE4_MOVEMENTCOMPONENT_AXIS_SETTINGS = 413,
    VER_UE4_GRAPH_INTERACTIVE_COMMENTBUBBLES = 414,
    VER_UE4_LANDSCAPE_SERIALIZE_PHYSICS_MATERIALS = 415,
    VER_UE4_RENAME_WIDGET_VISIBILITY = 416,
    VER_UE4_ANIMATION_ADD_TRACKCURVES = 417,
    VER_UE4_MONTAGE_BRANCHING_POINT_REMOVAL = 418,
    VER_UE4_BLUEPRINT_ENFORCE_CONST_IN_FUNCTION_OVERRIDES = 419,
    VER_UE4_ADD_PIVOT_TO_WIDGET_COMPONENT = 420,
    VER_UE4_PAWN_AUTO_POSSESS_AI = 421,
    VER_UE4_FTEXT_HISTORY_DATE_TIMEZONE = 422,
    VER_UE4_SORT_ACTIVE_BONE_INDICES = 423,
    VER_UE4_PERFRAME_MATERIAL_UNIFORM_EXPRESSIONS = 424,
    VER_UE4_MIKKTSPACE_IS_DEFAULT = 425,
    VER_UE4_LANDSCAPE_GRASS_COOKING = 426,
    VER_UE4_FIX_SKEL_VERT_ORIENT_MESH_PARTICLES = 427,
    VER_UE4_LANDSCAPE_STATIC_SECTION_OFFSET = 428,
    VER_UE4_ADD_MODIFIERS_RUNTIME_GENERATION = 429,
    VER_UE4_MATERIAL_MASKED_BLENDMODE_TIDY = 430,
    VER_UE4_MERGED_ADD_MODIFIERS_RUNTIME_GENERATION_TO_4_7_DEPRECATED = 431,
    VER_UE4_AFTER_MERGED_ADD_MODIFIERS_RUNTIME_GENERATION_TO_4_7_DEPRECATED = 432,
    VER_UE4_MERGED_ADD_MODIFIERS_RUNTIME_GENERATION_TO_4_7 = 433,
    VER_UE4_AFTER_MERGING_ADD_MODIFIERS_RUNTIME_GENERATION_TO_4_7 = 434,
    VER_UE4_SERIALIZE_LANDSCAPE_GRASS_DATA = 435,
    VER_UE4_OPTIONALLY_CLEAR_GPU_EMITTERS_ON_INIT = 436,
    VER_UE4_SERIALIZE_LANDSCAPE_GRASS_DATA_MATERIAL_GUID = 437,
    VER_UE4_BLUEPRINT_GENERATED_CLASS_COMPONENT_TEMPLATES_PUBLIC = 438,
    VER_UE4_ACTOR_COMPONENT_CREATION_METHOD = 439,
    VER_UE4_K2NODE_EVENT_MEMBER_REFERENCE = 440,
    VER_UE4_STRUCT_GUID_IN_PROPERTY_TAG = 441,
    VER_UE4_REMOVE_UNUSED_UPOLYS_FROM_UMODEL = 442,
    VER_UE4_REBUILD_HIERARCHICAL_INSTANCE_TREES = 443,
    VER_UE4_PACKAGE_SUMMARY_HAS_COMPATIBLE_ENGINE_VERSION = 444,
    VER_UE4_TRACK_UCS_MODIFIED_PROPERTIES = 445,
    VER_UE4_LANDSCAPE_SPLINE_CROSS_LEVEL_MESHES = 446,
    VER_UE4_DEPRECATE_USER_WIDGET_DESIGN_SIZE = 447,
    VER_UE4_ADD_EDITOR_VIEWS = 448,
    VER_UE4_FOLIAGE_WITH_ASSET_OR_CLASS = 449,
    VER_UE4_BODYINSTANCE_BINARY_SERIALIZATION = 450,
    VER_UE4_SERIALIZE_BLUEPRINT_EVENTGRAPH_FASTCALLS_IN_UFUNCTION = 451,
    VER_UE4_INTERPCURVE_SUPPORTS_LOOPING = 452,
    VER_UE4_MATERIAL_INSTANCE_BASE_PROPERTY_OVERRIDES_DITHERED_LOD_TRANSITION = 453,
    VER_UE4_SERIALIZE_LANDSCAPE_ES2_TEXTURES = 454,
    VER_UE4_CONSTRAINT_INSTANCE_MOTOR_FLAGS = 455,
    VER_UE4_SERIALIZE_PINTYPE_CONST = 456,
    VER_UE4_LIBRARY_CATEGORIES_AS_FTEXT = 457,
    VER_UE4_SKIP_DUPLICATE_EXPORTS_ON_SAVE_PACKAGE = 458,
    VER_UE4_SERIALIZE_TEXT_IN_PACKAGES = 459,
    VER_UE4_ADD_BLEND_MODE_TO_WIDGET_COMPONENT = 460,
    VER_UE4_NEW_LIGHTMASS_PRIMITIVE_SETTING = 461,
    VER_UE4_REPLACE_SPRING_NOZ_PROPERTY = 462,
    VER_UE4_TIGHTLY_PACKED_ENUMS = 463,
    VER_UE4_ASSET_IMPORT_DATA_AS_JSON = 464,
    VER_UE4_TEXTURE_LEGACY_GAMMA = 465,
    VER_UE4_ADDED_NATIVE_SERIALIZATION_FOR_IMMUTABLE_STRUCTURES = 466,
    VER_UE4_DEPRECATE_UMG_STYLE_OVERRIDES = 467,
    VER_UE4_STATIC_SHADOWMAP_PENUMBRA_SIZE = 468,
    VER_UE4_NIAGARA_DATA_OBJECT_DEV_UI_FIX = 469,
    VER_UE4_FIXED_DEFAULT_ORIENTATION_OF_WIDGET_COMPONENT = 470,
    VER_UE4_REMOVED_MATERIAL_USED_WITH_UI_FLAG = 471,
    VER_UE4_CHARACTER_MOVEMENT_ADD_BRAKING_FRICTION = 472,
    VER_UE4_BSP_UNDO_FIX = 473,
    VER_UE4_DYNAMIC_PARAMETER_DEFAULT_VALUE = 474,
    VER_UE4_STATIC_MESH_EXTENDED_BOUNDS = 475,
    VER_UE4_ADDED_NON_LINEAR_TRANSITION_BLENDS = 476,
    VER_UE4_AO_MATERIAL_MASK = 477,
    VER_UE4_NAVIGATION_AGENT_SELECTOR = 478,
    VER_UE4_MESH_PARTICLE_COLLISIONS_CONSIDER_PARTICLE_SIZE = 479,
    VER_UE4_BUILD_MESH_ADJ_BUFFER_FLAG_EXPOSED = 480,
    VER_UE4_MAX_ANGULAR_VELOCITY_DEFAULT = 481,
    VER_UE4_APEX_CLOTH_TESSELLATION = 482,
    VER_UE4_DECAL_SIZE = 483,
    VER_UE4_KEEP_ONLY_PACKAGE_NAMES_IN_STRING_ASSET_REFERENCES_MAP = 484,
    VER_UE4_COOKED_ASSETS_IN_EDITOR_SUPPORT = 485,
    VER_UE4_DIALOGUE_WAVE_NAMESPACE_AND_CONTEXT_CHANGES = 486,
    VER_UE4_MAKE_ROT_RENAME_AND_REORDER = 487,
    VER_UE4_K2NODE_VAR_REFERENCEGUIDS = 488,
    VER_UE4_SOUND_CONCURRENCY_PACKAGE = 489,
    VER_UE4_USERWIDGET_DEFAULT_FOCUSABLE_FALSE = 490,
    VER_UE4_BLUEPRINT_CUSTOM_EVENT_CONST_INPUT = 491,
    VER_UE4_USE_LOW_PASS_FILTER_FREQ = 492,
    VER_UE4_NO_ANIM_BP_CLASS_IN_GAMEPLAY_CODE = 493,
    VER_UE4_SCS_STORES_ALLNODES_ARRAY = 494,
    VER_UE4_FBX_IMPORT_DATA_RANGE_ENCAPSULATION = 495,
    VER_UE4_CAMERA_COMPONENT_ATTACH_TO_ROOT = 496,
    VER_UE4_INSTANCED_STEREO_UNIFORM_UPDATE = 497,
    VER_UE4_STREAMABLE_TEXTURE_MIN_MAX_DISTANCE = 498,
    VER_UE4_INJECT_BLUEPRINT_STRUCT_PIN_CONVERSION_NODES = 499,
    VER_UE4_INNER_ARRAY_TAG_INFO = 500,
    VER_UE4_FIX_SLOT_NAME_DUPLICATION = 501,
    VER_UE4_STREAMABLE_TEXTURE_AABB = 502,
    VER_UE4_PROPERTY_GUID_IN_PROPERTY_TAG = 503,
    VER_UE4_NAME_HASHES_SERIALIZED = 504,
    VER_UE4_INSTANCED_STEREO_UNIFORM_REFACTOR = 505,
    VER_UE4_COMPRESSED_SHADER_RESOURCES = 506,
    VER_UE4_PRELOAD_DEPENDENCIES_IN_COOKED_EXPORTS = 507,
    VER_UE4_TemplateIndex_IN_COOKED_EXPORTS = 508,
    VER_UE4_PROPERTY_TAG_SET_MAP_SUPPORT = 509,
    VER_UE4_ADDED_SEARCHABLE_NAMES = 510,
    VER_UE4_64BIT_EXPORTMAP_SERIALSIZES = 511,
    VER_UE4_SKYLIGHT_MOBILE_IRRADIANCE_MAP = 512,
    VER_UE4_ADDED_SWEEP_WHILE_WALKING_FLAG = 513,
    VER_UE4_ADDED_SOFT_OBJECT_PATH = 514,
    VER_UE4_POINTLIGHT_SOURCE_ORIENTATION = 515,
    VER_UE4_ADDED_PACKAGE_SUMMARY_LOCALIZATION_ID = 516,
    VER_UE4_FIX_WIDE_STRING_CRC = 517,
    VER_UE4_ADDED_PACKAGE_OWNER = 518,
    VER_UE4_SKINWEIGHT_PROFILE_DATA_LAYOUT_CHANGES = 519,
    VER_UE4_NON_OUTER_PACKAGE_IMPORT = 520,
    VER_UE4_ASSETREGISTRY_DEPENDENCYFLAGS = 521,
    VER_UE4_CORRECT_LICENSEE_FLAG = 522,
}
