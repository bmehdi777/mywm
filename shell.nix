let 
	pkgs = import <nixpkgs> {};
in

pkgs.mkShell {
	packages = with pkgs; [
		xorg.xorgserver
		xorg.xev
	];
}
