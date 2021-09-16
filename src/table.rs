use crate::SqlBuilder;

pub trait Table {
    const NAME: &'static str;
    
    fn columns() -> Vec<&'static str>;
}

pub trait Model {
    type Table: Table;

    fn include() -> Vec<&'static str> {
        vec![]
    }

    fn exclude() -> Vec<&'static str> {
        vec![]
    }

    fn fields() -> Vec<&'static str> {
        let fields = Self::Table::columns();
        let include = Self::include();
        let exclude = Self::exclude();

        if include.len() == 0 && exclude.len() == 0 {
            return fields;
        } else if exclude.len() == 0 {
            return include;
        } else {
            fields
                .into_iter()
                .filter(|field| exclude.iter().all(|item| **item != **field))
                .collect()
        }
    }
}

pub trait SqlDsl: Model {
    fn select() -> SqlBuilder {
        let fields = Self::fields();
        let name = <<Self as Model>::Table as Table>::NAME;
        let mut sql_builder = SqlBuilder::select_from(name);
        sql_builder.fields(&fields);
        sql_builder
    }

    fn insert() -> SqlBuilder {
        let fields = Self::fields();
        let name = <<Self as Model>::Table as Table>::NAME;
        let mut sql_builder = SqlBuilder::insert_into(name);
        sql_builder.fields(&fields);
        sql_builder
    }

    fn update() -> SqlBuilder {
        let name = <<Self as Model>::Table as Table>::NAME;
        let sql_builder = SqlBuilder::update_table(name);
        sql_builder
    }

    fn delete() -> SqlBuilder {
        let name = <<Self as Model>::Table as Table>::NAME;
        let sql_builder = SqlBuilder::delete_from(name);
        sql_builder
    }
}

impl<T: Model> SqlDsl for T {}
