use snowflaked::sync::Generator;

static GENERATOR: Generator = Generator::new(0);

pub fn generate_id() -> i64 {
    GENERATOR.generate()
}
