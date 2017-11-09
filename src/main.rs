extern crate commander;

mod shps;

use commander::Commander;

use shps::SHPS;


fn main() {
    let command = Commander::new()
        .version(&env!("CARGO_PKG_VERSION").to_string())
        .usage("SHPS")
        .usage_desc("Start SHPS data-pipeline")
        .option("-d, --debug", "Enable debug", None)
        .parse_env_or_exit()
    ;

    // todo: add objects to SHPS
    //  - signalling (using event/message system)
    //  - commandline (using custom system)
    //  - logging (using slog with derived loggers per component)
    //  - config management (using serde and ron)
    //  - plugin engine
    //  - db abstraction (using Diesel)
    //  - authentication (custom security contexts)
    //  - distributed computing (multi-threading
    //  - pipeline (using chum)
    //  - cache
    //  - session manager
    //  - JS interpreter for simple scripting
    //  - i10n/i18n
    let core = SHPS::new(command.get("d").is_some());

    // todo: use config module to read SHPS base config
}
