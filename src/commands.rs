use std::fmt::Write;

use anyhow::bail;

const DEFAULT_NUMBER_OF_ROWS: usize = 5;
const DEFAULT_SEPARATOR: &str = ",";

pub fn view(file: String, rows: Option<usize>, column_names: bool) -> String {
    let rows_to_skip = if column_names { 0 } else { 1 };
    let rows_to_return = if column_names {
        // Since the column names are returned "1" must be added to compensate
        // for that.
        rows.unwrap_or(DEFAULT_NUMBER_OF_ROWS) + 1
    } else {
        rows.unwrap_or(DEFAULT_NUMBER_OF_ROWS)
    };

    file.lines()
        .skip(rows_to_skip)
        .take(rows_to_return)
        .fold(String::new(), |mut acc, l| {
            writeln!(acc, "{}", l).unwrap();
            acc
        })
}

pub fn filter(
    file: String,
    column: String,
    equals: String,
    separator: Option<String>,
) -> anyhow::Result<String> {
    let separator = separator.unwrap_or(DEFAULT_SEPARATOR.to_string());

    // TODO: Look for solutions that don't involve calling `lines()`
    let col_idx = match file.lines().next() {
        Some(columns) => {
            match columns
                .split(&separator)
                .enumerate()
                .find(|(_, col)| *col == column)
            {
                Some((idx, _)) => idx,
                None => bail!("Column does not exist"),
            }
        }
        None => bail!("The file is empty"),
    };

    Ok(file
        .lines()
        // The `unwrap()` is safe because of the previous step that computes `col_idx`
        .filter(|row| row.split(&separator).nth(col_idx).unwrap() == equals)
        .fold(String::new(), |mut acc, l| {
            writeln!(acc, "{}", l).unwrap();
            acc
        }))
}

#[cfg(test)]
mod tests {
    use crate::commands::{self, DEFAULT_NUMBER_OF_ROWS};

    macro_rules! assert_err {
        ($result:expr, $expected_message:expr) => {
            match $result {
                Ok(_) => panic!("Expected error"),
                Err(err) => {
                    assert_eq!(err.to_string(), $expected_message);
                }
            }
        };
    }

    const DATA: &str = r"name,email,date_of_birth,score,city
Kyle George,michaeldavis@example.com,1986-11-23,57,Brendaside
Robin Payne,snydertimothy@example.com,1988-12-21,20,Derrickstad
Bruce Morgan,deborah75@example.org,1962-01-25,100,Cynthiastad
Jacob Watson,hjones@example.com,1983-12-06,72,Roymouth
Matthew Hernandez,phillipjones@example.com,1983-08-06,34,Mitchellburgh
Robin Moran,thomasabigail@example.net,1977-10-04,20,Harrisville
Brian Mcdonald,thomasjeffrey@example.com,1970-09-10,69,Wilsonfort
Amber Johnson,jonesdale@example.org,1977-11-09,3,Blairberg
Jordan Brown,atanner@example.com,1960-05-29,75,North Lauramouth
Veronica Taylor,ifrank@example.net,2001-04-11,83,Port Morgan";

    #[test]
    fn view_returns_the_default_number_of_rows() {
        let res = commands::view(DATA.to_string(), None, false);
        assert_eq!(res.lines().count(), DEFAULT_NUMBER_OF_ROWS);
    }

    #[test]
    fn view_returns_the_specified_number_of_rows() {
        let res = commands::view(DATA.to_string(), Some(7), false);
        assert_eq!(res.lines().count(), 7);
    }

    #[test]
    fn view_returns_data_with_column_names() {
        let res = commands::view(DATA.to_string(), None, true);
        assert_eq!(res.lines().count(), DEFAULT_NUMBER_OF_ROWS + 1);
    }

    #[test]
    fn view_returns_data_without_column_names() {
        let res = commands::view(DATA.to_string(), None, false);
        assert_eq!(res.lines().count(), DEFAULT_NUMBER_OF_ROWS);
    }

    #[test]
    fn filter_errors_if_the_column_does_not_exist() {
        let res = commands::filter(
            DATA.to_string(),
            "non-existent".to_string(),
            "abc".to_string(),
            None,
        );

        assert_err!(res, "Column does not exist")
    }

    #[test]
    fn filter_errors_if_the_file_is_empty() {
        let res = commands::filter(
            "".to_string(),
            "non-existent".to_string(),
            "abc".to_string(),
            None,
        );

        assert_err!(res, "The file is empty")
    }

    #[test]
    fn filter_returns_the_rows_that_match_the_search() {
        let res = commands::filter(
            DATA.to_string(),
            "name".to_string(),
            "Jordan Brown".to_string(),
            None,
        );

        assert_eq!(
            res.unwrap(),
            "Jordan Brown,atanner@example.com,1960-05-29,75,North Lauramouth\n"
        )
    }

    #[test]
    fn filter_returns_the_rows_that_match_the_search_using_a_specified_separator() {
        let data = r"name;email;date_of_birth;score;city
Kyle George;michaeldavis@example.com;1986-11-23;57;Brendaside
Robin Payne;snydertimothy@example.com;1988-12-21;20;Derrickstad
Bruce Morgan;deborah75@example.org;1962-01-25;100;Cynthiastad
Jacob Watson;hjones@example.com;1983-12-06;72;Roymouth
Matthew Hernandez;phillipjones@example.com;1983-08-06;34;Mitchellburgh
Robin Moran;thomasabigail@example.net;1977-10-04;20;Harrisville
Brian Mcdonald;thomasjeffrey@example.com;1970-09-10;69;Wilsonfort
Amber Johnson;jonesdale@example.org;1977-11-09;3;Blairberg
Jordan Brown;atanner@example.com;1960-05-29;75;North Lauramouth
Veronica Taylor;ifrank@example.net;2001-04-11;83;Port Morgan";

        let res = commands::filter(
            data.to_string(),
            "name".to_string(),
            "Jordan Brown".to_string(),
            Some(";".to_string()),
        );

        assert_eq!(
            res.unwrap(),
            "Jordan Brown;atanner@example.com;1960-05-29;75;North Lauramouth\n"
        )
    }
}
