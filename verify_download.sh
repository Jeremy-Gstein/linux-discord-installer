#!/usr/bin/bash

ARCH="x86_64-musl"
TAG="v0.1.0"
BASE="https://github.com/Jeremy-Gstein/linux-discord-installer/releases/download/$TAG"
BINARY="linux-discord-installer-$ARCH"
SHA="$BINARY.sha256"
SIG="$SHA.asc"

spinner() {
  local seconds="${1:-5}"
  local delay=0.1
  local spin='-\|/'
  local end=$((SECONDS + seconds))

  while (( SECONDS < end )); do
    for (( i=0; i<${#spin} && SECONDS < end; i++ )); do
      printf "\r[%c]" "${spin:i:1}"
      sleep "$delay"
    done
  done
  printf "\r   \r"
}

download_release() { 
  echo "getting: $1..."
  spinner 1
  curl -sLO "$1" || { echo "Download failed"; exit 1; }
}

get_release() {
  download_release "$BASE/$BINARY"
  download_release "$BASE/$SHA"
  download_release "$BASE/$SIG"
}

verify() {
  get_release
  spinner 1
  echo "Verifying gpg: $SIG"
  gpg --verify "$SIG" "$SHA" || { echo "GPG verification failed"; exit 1; }
  echo "Verifying: $SHA"
  sha256sum -c "$SHA" || { echo "SHA256 verification failed"; exit 1; }
  echo "All verifications passed!"
}

get_installer() {
  pushd $PWD
  cd /tmp
  spinner 1
  verify && \
    echo "linux-discord-installer -> /tmp/$BINARY"
  spinner 1
  chmod +x "/tmp/$BINARY"
  spinner 1
  /tmp/$BINARY --help
  spinner 1
  popd
}

clean_cwd() {
  rm -f "$BINARY"
  spinner 1
  rm -f "$SHA"
  spinner 1
  rm -f "$SIG"
}

help() {
  cat << 'EOF'
verify_download.sh 
                    -   verify linux-discord-installer releases   -

USAGE:
  ./verify_download.sh [-v|--verify] [-t TAG|--tag TAG] [-i|--install] [-c|--clean] [-h|--help]
    
  --verify)  Gets the defualt (or specified) tagged release)

  --tag TAG) Specify release tag to get 

  --install) Download and verify static binary to /tmp/$BINARY

  --clean)   Removes the binary, sig, and sha from cwd

  --help)    Prints this menu

EXAMPLES:
  Work in $PWD:
    ./verify_download.sh --verify
    ./verify_download.sh -t v0.1.1 --verify
  Work in /tmp:
    ./verify_download.sh --install
    ./verify_download.sh -t v0.1.1 --install
EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    -t|--tag)
      shift
      [[ -z "$1" ]] && { echo "Error: --tag requires value"; exit 1; }
      TAG="$1"
      BASE="https://github.com/Jeremy-Gstein/linux-discord-installer/releases/download/$TAG"
      BINARY="linux-discord-installer-$ARCH"
      SHA="$BINARY.sha256"
      SIG="$SHA.asc"
      shift
      ;;
    -v|--verify) verify; shift ;;
    -c|--clean) clean_cwd; exit 0 ;;
    -i|--install) get_installer; exit 0 ;;
    -h|--help) help; exit 0 ;;
    *) echo "Unknown argument: $1"; help; exit 1 ;;
  esac
done

help
