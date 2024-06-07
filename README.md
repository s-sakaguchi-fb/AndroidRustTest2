# Android Rust JNI Test

## 環境構築方法
### 1. Rustのインストール(未実施であれば)
```
brew install rustup-init
rustup-init
```
<br>

### 2. Androidターゲットのインストール
```
rustup target add aarch64-linux-android x86_64-linux-android
```
<br>

### 3. Android NDKのインストール
Android StudioからTools→SDK Managerを開き、SDK ToolsのNDKをインストールする。\
<br>

### 4. CargoにAndroid NDKのパスを設定
SDK ManagerのウィンドウからAndroid SDK Locationを調べ、この中にあるndk-bundleまたはndk/<バージョン>フォルダを確認する。\
~/.cargo/config.tomlに、確認したフォルダの場所を記載。
```
[target.aarch64-linux-android]
linker = "（Android NDKへのパス）/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android35-clang"
```
<br>

### 5. pythonコマンドが実行可能か確認する
ターミナルやコマンドプロンプトで
```
python
```
が実行可能か確認。`python`は実行できないが`python3`は実行可能な場合、シンボリックリンクを貼るなどで`python`コマンドを実行可能とする必要がある。

## ファイル構成
* app
  * src
    * java - Androidアプリメイン
    * lib - 動作に必要な共有ライブラリを格納
    * rust - Rust JNIプロジェクト一式をここに格納
 
