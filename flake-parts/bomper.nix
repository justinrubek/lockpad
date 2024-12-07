{inputs, ...}: {
  imports = [inputs.bomper.flakeModules.bomper];
  perSystem = _: {
    bomper = {
      enable = true;
      configuration = ''
        (
            cargo: Some(Autodetect),
            authors: Some({
                "Justin Rubek": "justinrubek"
            }),
        )
      '';
    };
  };
}
