<h1>Image2Jpeg</h1>
<p>This tool made by Rust provides amazing experience of creating pdf from jpeg, png and avif!</p>
<p>Just run 'cargo run [path]' and get PDF.</p>

> [!NOTE]
> This tool doesn't repair SOI Maker.

<h1>Features</h1>
<ul>
  <li>Made by Rust</li>
  <li>Get command line args</li>
  <li>Supproted format is jpeg, png and avif</li>
  <li>Asynchronous</li>
  <li>Run on Tokio Runtime</li>
</ul>

<h1>How to use</h1>
<ol>
  <li>
    Install Rust
    <pre><co>$ curl https://sh.rustup.rs -sSf | sh</code></pre>
    Add PATH follow the install message
  </li>
  <li>
    Install <code>nasm</code> and <code>dav1d</code>
    <p><b>Debian:</b></p>
    <pre><code>$ sudo apt update<br>$ sudo apt install nasm dav1d-devel pkg-config</code></pre>
    <p><b>Fedora:</b></p>
    <pre><code>$ sudo dnf update<br>$ sudo dnf install nasm dav1d-devel pkg-config</code></pre>
    <p><b>Arch Linux:</b></p>
    <pre><code>$ sudo pacman -Syyu<br>$ sudo pacman -S nasm dav1d pkgconf</code></pre>
  </li>
  <li>
    Add PKG_CONFIG_PATH<br>
    To get dav1d's PATH
    <pre><code>pkg-config --variable=pcfiledir dav1d</code></pre>
  </li>
  <li>
    <code>git clone</code>
    <pre><code>$ git clone https://github.com/hihimamuLab/Image2Pdf.git<br>$ cd Image2Pdf</code></pre>
  </li>
  <li>
    <code>cargo run</code>
    <pre><code>$ cargo run [Path of dir that has images]</code></pre>
  </li>
</ol>
