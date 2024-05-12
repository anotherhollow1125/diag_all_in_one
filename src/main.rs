use anyhow::Result;
use console::Term;
use dialoguer::{BasicHistory, Confirm, Editor, FuzzySelect, Input, MultiSelect, Password, Sort};

fn confirm_demo() -> Result<()> {
    println!(
        "
【Confirm】

[y/n] 形式で質問する
"
    );

    let confirmation = Confirm::new()
        .with_prompt("縮小用廃棄物投入プロトコルを実施しますか?")
        .show_default(true)
        .wait_for_newline(true)
        .interact()?;

    if confirmation {
        println!("プロトコルを開始しました");
    } else {
        println!("中止しました");
    }

    Ok(())
}

fn fuzzyselect_demo() -> Result<()> {
    println!(
        "
【FuzzySelect】

入力による選択で、徐々に絞り込まれていく
"
    );

    let items = ["Python", "R", "Rust", "Ruby", "Ruby on Rails"];

    let selection = FuzzySelect::new()
        .with_prompt("What do you use?")
        .items(&items)
        .interact()?;

    println!("You chose: {}", items[selection]);

    Ok(())
}

fn multiselect_demo() -> Result<()> {
    println!(
        "
【MultiSelect】

チェックボックスによる複数選択ができる
"
    );

    let items = [
        "Basic",
        "Extension for VSCode",
        "Extension for Rust",
        "Utility for Git",
    ];

    let selections = MultiSelect::new()
        .with_prompt("インストールする項目を選んでください (スペースで選択/解除)")
        .item_checked(items[0], true)
        .items(&items[1..])
        .interact()?;

    println!("以下をインストールします:");
    for selection in selections {
        println!("{}", items[selection]);
    }

    Ok(())
}

fn sort_demo() -> Result<()> {
    println!(
        "
【Sort】

ユーザーによりソートされたインデックスを返す
"
    );

    let items = vec!["SSD-1", "HDD-1", "HDD-2", "USB", "DVD"];

    let order = Sort::new()
        .with_prompt("Boot Device Priority (Space key: select, Arrow keys: move)")
        .items(&items)
        .interact()?;

    let mut new_priority: Vec<_> = order
        .into_iter()
        .enumerate()
        .map(|(priority, i)| (priority, items[i]))
        .collect();

    new_priority.sort_by_key(|(priority, _)| *priority);

    let boot_devices = new_priority
        .iter()
        .map(|(_, item)| item)
        .collect::<Vec<_>>();

    println!("Sorted Vector: {:?}", boot_devices);

    Ok(())
}

fn password_demo() -> Result<()> {
    println!(
        "
【Password】

Inputと同様に入力を受け付けるが、入力内容を表示しない
"
    );

    let password = Password::new()
        .with_prompt("Enter your password")
        .interact()?;

    let password_confirm = Password::new()
        .with_prompt("Confirm your password")
        .interact()?;

    if password == password_confirm {
        println!("Password is confirmed");
    } else {
        println!("Password is not confirmed");
    }

    Ok(())
}

fn history_demo() -> Result<()> {
    println!(
        "
【History Trait (BasicHistory)】

過去の入力を履歴として保存する。
矢印キーにより過去の入力を呼び出せる
"
    );

    let mut history = BasicHistory::new().no_duplicates(true);

    loop {
        let input: String = Input::new()
            .with_prompt("Enter something")
            .history_with(&mut history)
            .interact_text()?;

        if input == "exit" {
            break;
        }
    }

    Ok(())
}

fn editor_demo() -> Result<()> {
    let content = Editor::new().edit(
        "
【Editor】

Vim, Emacsなどのデフォルトエディタを開き、入力を受け付ける
",
    )?;

    println!(
        "Edited content:
{}
",
        content.unwrap_or("None".to_string())
    );

    Ok(())
}

fn main() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    confirm_demo()?;
    let _ = term.read_key()?;
    term.clear_screen()?;

    fuzzyselect_demo()?;
    let _ = term.read_key()?;
    term.clear_screen()?;

    multiselect_demo()?;
    let _ = term.read_key()?;
    term.clear_screen()?;

    sort_demo()?;
    let _ = term.read_key()?;
    term.clear_screen()?;

    password_demo()?;
    let _ = term.read_key()?;
    term.clear_screen()?;

    history_demo()?;
    let _ = term.read_key()?;
    term.clear_screen()?;

    editor_demo()?;

    Ok(())
}
