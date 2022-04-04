mod interpretor;

fn main() {
    let mut interpretor = interpretor::Interpretor::new(
        interpretor::text_to_operations("
            LOAD_VAL 1
            WRITE_VAR 'x'
            LOAD_VAL 2
            WRITE_VAR 'y'
            READ_VAR 'x'
            LOAD_VAL 1
            ADD
            READ_VAR 'y'
            MULTIPLY
            RETURN_VALUE
            LOAD_VAL 0
            WRITE_VAR 'x'
            LOAD_VAL 0
            WRITE_VAR 'i'
            READ_VAR 'i'
            TEST_DIFFERENT_FROM 5
            JUMP_IF_FALSE 26
            READ_VAR 'x'
            LOAD_VAL 10
            ADD
            WRITE_VAR 'x'
            READ_VAR 'i'
            LOAD_VAL 1
            ADD
            WRITE_VAR 'i'
            JUMP_LOOP 14
            READ_VAR 'x'
            RETURN_VALUE
        ")
    );

    interpretor.interpret();
}