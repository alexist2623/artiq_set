{ fetchFromGitHub, python3Packages }:
with python3Packages;

buildPythonPackage rec {
  pname = "mattermost-github-integration";
  version = "0.0.0-unstable";
  src = fetchFromGitHub {
    owner = "softdevteam";
    repo = "mattermost-github-integration";
    rev = "master";
    sha256 = "1hfvjaxjhliy8sv9j3616fkdwd2jqhfsj9ai7ggx88zhxknrfx85";
  };
  propagatedBuildInputs = [
    appdirs
    click
    flask
    itsdangerous
    jinja2
    markupsafe
    olefile
    packaging
    pillow
    pyparsing
    requests
    six
    werkzeug
  ];
  checkInputs = [
    pytest
  ];
  doCheck = true;
}
