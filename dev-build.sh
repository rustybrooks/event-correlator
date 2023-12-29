if [ ! -d ./venv ]; then
    echo "Making virtualenv"
    virtualenv ./venv
fi


if [[ -z "$VIRTUAL_ENV" ]]; then
    echo "No VIRTUAL_ENV set"
    . ./venv/bin/activate
else
    echo "VIRTUAL_ENV is set"
fi

maturin build && pip install --force-reinstall target/wheels/event_correlator-0.1.0-cp310-cp310-macosx_11_0_arm64.whl
