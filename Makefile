# var
MODULE = $(notdir $(CURDIR))
module = $(shell echo $(MODULE) | tr A-Z a-z)
OS     = $(shell uname -s)
NOW    = $(shell date +%d%m%y)
REL    = $(shell git rev-parse --short=4 HEAD)
BRANCH = $(shell git rev-parse --abbrev-ref HEAD)

# version
JQUERY_VER  = 3.6.1
POPPER_VER  = 2.11.6
BS_DARK_VER = 5.2.2
BS_VER      = 5.2.3

# target
WASM   = wasm32-unknown-unknown

# dir
CWD    = $(CURDIR)
CAR    = $(HOME)/.cargo/bin
CDN    = https://cdnjs.cloudflare.com/ajax/libs

# tool
CURL   = curl -L -o
CARGO  = $(CAR)/cargo
RUSTUP = $(CAR)/rustup
CF     = clang-format

# src
R += $(shell find src    -regex '.+.rs$$')
R += $(shell find common -regex '.+.rs$$')
R += $(shell find config -regex '.+.rs$$')
R += $(shell find vm     -regex '.+.rs$$')
R += $(shell find server -regex '.+.rs$$')
R += $(shell find wasm   -regex '.+.rs$$')
R += $(shell find cortex -regex '.+.rs$$')
R += $(shell find cad    -regex '.+.rs$$')
R += $(shell find wam    -regex '.+.rs$$')
R += $(shell find eco    -regex '.+.rs$$')
T += $(shell find        -regex '.+.toml$$')
S += $(F) $(R) $(T)

JS += $(shell find static -maxdepth 1 -regex '.+.js$$')
JS += $(shell find cad    -maxdepth 1 -regex '.+.js$$')

# all
.PHONY: all
all: format
#	$(CARGO) build --workspace \
#		--exclude wasm --exclude cortex --exclude droid

.PHONY: cortex
cortex:
	$(CARGO) build -p $@

.PHONY: droid
droid:
	$(CARGO) build -p $@ --target armv7-linux-androideabi

.PHONY: nocore
nocore:
	RUSTFLAGS="-C link-args=-nostartfiles" \
		$(CARGO) +nightly watch -w $@ \
		-x 'fmt -p $@' -x 'build -p $@' -x 'run -p $@'

.PHONY: server
server:
	$(CARGO) watch --ignore-nothing -w $@ -w Cargo.toml -w $@/Cargo.toml \
		-w config -w common -w static -w target/$(WASM) \
		-x 'fmt' -x 'run -p $@'

.PHONY: eco
eco:
	$(CARGO) watch -w $@ -w Cargo.toml -w config -w common -x 'run -p $@'

.PHONY: bench
bench:
	ab -q -n 11111 -c 11 http://127.0.0.1:12345/ > tmp/$@

.PHONY: cad
cad:
#	$(CARGO) +nightly watch -w $@ -w Cargo.toml
	$(CARGO) watch -w $@ -w Cargo.toml -i $@/pkg \
		-s "wasm-pack build $@ --target web"
#		-x 'build --target=$(WASM) -p $@'

tmp/cad.wat: cad/pkg/cad_bg.wasm
# target/$(WASM)/debug/cad.wasm
	wasm2wat $< -o $@

.PHONY: wam
wam:
	$(CARGO) watch \
		-w $@ -w Cargo.toml -w config -w common \
		-x 'fmt -p $@' -x 'run -p $@'

.PHONY: wasm
wasm:
	$(CARGO) build -p $@

# format
.PHONY: format
format: tmp/format_rs tmp/format_js
tmp/format_rs: $(R)
	$(CARGO) fmt --all && touch $@
tmp/format_js: $(JS)
	$(CF) --style=file -i $? && touch $@

# doc
.PHONY: doc
doc: \
	doc/rustbook_ru.pdf \
	doc/wambook.pdf doc/wam-slides.pdf
#
	rsync -rvd ~/mdoc/$(MODULE)/* doc/$(MODULE)/
	rsync -rvd ~/mdoc/WASM/*      doc/WASM/
	rsync -rvd ~/mdoc/Rust/*      doc/Rust/
	rsync -rvd ~/mdoc/net/*       doc/net/
	rsync -rvd ~/mdoc/HTTP/*      doc/HTTP/
	git add doc

.PHONY: doxy
doxy: .doxygen
	rm -rf docs ; doxygen $< 1>/dev/null
	cargo doc --workspace --no-deps && mv target/doc docs/rust

doc/rustbook_ru.pdf:
	$(CURL) $@ https://raw.githubusercontent.com/kgv/rust_book_ru/gh-pages/converted/rustbook.pdf

doc/wam-slides.pdf:
	$(CURL) $@ https://github.com/a-yiorgos/wambook/raw/master/wam-slides.pdf
doc/wambook.pdf:
	$(CURL) $@ https://github.com/a-yiorgos/wambook/raw/master/wambook.pdf

# install
.PHONY: install
install: $(OS)_install doc gz rust
	$(RUSTUP) target  add wasm32-unknown-unknown
	$(RUSTUP) target  add thumbv6m-none-eabi
	$(RUSTUP) target  add armv7-linux-androideabi
	$(RUSTUP) install nightly
	$(CARGO)  install cargo-watch wasm-bindgen-cli wasm-pack
	$(MAKE)   update

.PHONY: update
update: $(OS)_update rust
	$(RUSTUP) self update && $(RUSTUP) update && $(CARGO) update

.PHONY: Linux_install Linux_update
Linux_install Linux_update:
	sudo apt update
	sudo apt install -yu `cat apt.txt apt.dev`

.PHONY: rust
rust: $(RUSTUP) $(CARGO)
$(RUSTUP) $(CARGO):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# js
.PHONY: gz js
gz: js
js: \
	static/cdn/jquery.min.js       static/cdn/popper.min.js       \
	static/cdn/bootstrap.min.js    static/cdn/darkly.min.css      \
	static/cdn/Lato.css            static/cdn/Lato_italic_400.ttf \
	static/cdn/Lato_normal_400.ttf static/cdn/Lato_normal_700.ttf

static/cdn/Lato_italic_400.ttf:
	$(CURL) $@ https://fonts.gstatic.com/s/lato/v23/S6u8w4BMUTPHjxswWw.ttf
static/cdn/Lato_normal_400.ttf:
	$(CURL) $@ https://fonts.gstatic.com/s/lato/v23/S6uyw4BMUTPHvxk.ttf
static/cdn/Lato_normal_700.ttf:
	$(CURL) $@ https://fonts.gstatic.com/s/lato/v23/S6u9w4BMUTPHh6UVew8.ttf

static/cdn/Lato.css:
	$(CURL) $@ "https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,400;0,700;1,400&display=swap"

static/cdn/jquery.min.js:
	$(CURL) $@ $(CDN)/jquery/$(JQUERY_VER)/jquery.min.js
static/cdn/popper.min.js:
	$(CURL) $@ $(CDN)/popper.js/$(POPPER_VER)/umd/popper.min.js
static/cdn/bootstrap.min.js:
	$(CURL) $@ $(CDN)/bootstrap/$(BS_VER)/js/bootstrap.min.js
static/cdn/darkly.min.css:
	$(CURL) $@ $(CDN)/bootswatch/$(BS_DARK_VER)/darkly/bootstrap.min.css


# merge
MERGE += README.md Makefile .gitignore apt.txt apt.dev LICENSE $(S)
MERGE += .vscode bin doc tmp src
MERGE += common config vm server wasm cortex droid cad wam eco
MERGE += static

.PHONY: main
main:
	git push -v
	git checkout $@
	git pull -v
	git checkout shadow -- $(MERGE)
.PHONY: shadow
shadow:
	git push -v
	git checkout $@
	git pull -v
.PHONY: release
release:
	git tag $(NOW)-$(REL)
	git push -v && git push -v --tags
	$(MAKE) shadow
.PHONY: zip
zip:
	git archive \
		--format zip \
		--output $(TMP)/$(MODULE)_$(NOW)_$(REL).src.zip \
	HEAD
