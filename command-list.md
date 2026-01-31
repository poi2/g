# 独自のコマンドリスト

## repository 系

基本、どこでも利用可能

- g sonic-repository clone <url>
    - 指定の dir に repository を clone する
- g sonic-repository ls
    - 指定の dir にある repository の一覧を表示
- g sonic-repository switch <repository>
    - 指定の dir にある指定の repository に移動
- g sonic-repository switch -i
    - 指定の dir にある repository を対話選択して移動
- g sonic-repository delete <repository>
    - 指定の dir にある指定の repository を削除
- g sonic-repository delete -i
    - 指定の dir にある指定の repository を対話選択して削除
- g sonic-repository new <repository>
    - 指定の dir に repository を作成

config で以下で実行可能にする。

- g rc <url>: sonic-repository clone <url>
- g rs: sonic-repository switch -i
- g rd: sonic-repository delete -i
- g rn <repository>: sonic-repository new <repository>

## switch 系

git 配下でのみ実行可能

- g sonic-switch -i
    - branch 一覧を表示し、incremental に branch を選択して、branch に移動

config で以下で実行可能にする。

- g si: sonic-switch -i
- g s <branch>: sonic-switch <branch>

## branch 系

git 配下でのみ実行可能

### 一覧

- g sonic-branch ls <options>
    - branch 一覧
    - git branch <options> に移譲
        - ただし CUD 系の option は error: unknown option

config で以下で実行可能にする。

- g bl <options>: sonic-branch ls <options>

### 作成

- g sonic-branch new <branch>
    - switch -c <branch>

config で以下で実行可能にする。

- g bn <branch>

### 変更

- g sonic-branch mv <old> <new>
    - old を new に変更
- g sonic-branch mv <new>
    - current を new に変更

config で以下で実行可能にする。

- g bm: sonic-branch mv

### 削除

- g sonic-branch delete <branch>
    - git branch -d <branch>
- g sonic-branch delete -f <branch>
    - git branch -D <branch>
- g sonic-branch delete -a
    - base/current 以外の branch を git branch -d で削除
- g sonic-branch delete -a -f
    - base/current 以外の branch を git branch -D で削除
- g sonic-branch delete -i
    - base/current 以外の merge 済み branch を対話選択して git branch -d で削除
- g sonic-branch delete -i -f
    - base/current 以外の branch を対話選択して git branch -D で削除

config で以下で実行可能にする。

- g bd: sonic-branch delete

## worktree 系

すべて current repository の worktree に対する実行。
異なる repository の worktree は対象外。

- g sonic-worktree
    - new <worktree>
        - worktree を作成
    - ls
        - worktree 一覧
    - mv <old> <new>
        - old を new に変更
    - mv <new>
        - current を new に変更
    - switch -i
        - 対話選択で worktree へ移動
    - switch <worktree>
        - worktree へ移動
    - delete -a
        - current 以外の worktree を worktree remove で削除
    - delete -a -f
        - current 以外の worktree を worktree remove -f で削除
    - delete -i
        - current 以外の worktree を対話選択で worktree remove で削除
    - delete -i -f
        - current 以外の worktree を対話選択で worktree remove -f で削除

config で以下で実行可能にする

- g wtn
- g wtl
- g wtm
- g wtsi: sonic-worktree switch -i
- g wts
- g wtd
