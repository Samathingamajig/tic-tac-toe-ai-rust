{ pkgs }: {
	deps = [
		pkgs.less
  pkgs.vim
  pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}