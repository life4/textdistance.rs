
from pathlib import Path
from typing import Iterator

import pytest


SKIP = frozenset({'algorithm', 'counter'})
ROOT = Path(__file__).parent.parent


def get_algorithms() -> Iterator[str]:
    for fpath in (ROOT / 'src' / 'textdistance').iterdir():
        alg_name = fpath.stem
        if alg_name not in SKIP:
            yield alg_name


ALGORITHMS = tuple(get_algorithms())


@pytest.mark.parametrize('alg', ALGORITHMS)
def test_name_is_ascii(alg: str) -> None:
    assert alg.replace('_', '').isalnum()
    assert alg.isascii()


@pytest.mark.parametrize('alg', ALGORITHMS)
def test_is_in_readme(alg: str) -> None:
    text = (ROOT / 'README.md').read_text()
    alg = alg.replace('_', '')
    assert f'1. `{alg}`' in text.lower()


@pytest.mark.parametrize('alg', ALGORITHMS)
def test_str_shortcut_exists(alg: str) -> None:
    fpath = (ROOT / 'src' / 'str.rs')
    assert f'fn {alg}(' in fpath.read_text()


@pytest.mark.parametrize('alg', ALGORITHMS)
def test_is_exported(alg: str) -> None:
    fpath = (ROOT / 'src' / 'lib.rs')
    text = fpath.read_text()
    assert f'mod {alg}' in text
    assert f'pub use self::{alg}::' in text


@pytest.mark.parametrize('alg', ALGORITHMS)
def test_is_smoke_tested(alg: str) -> None:
    fpath = (ROOT / 'src' / 'lib.rs')
    text = fpath.read_text()
    assert f'#[case::{alg}(' in text


@pytest.mark.parametrize('alg', ALGORITHMS)
def test_is_tested(alg: str) -> None:
    fpath = (ROOT / 'src' / 'textdistance' / f'{alg}.rs')
    assert '#[rstest]' in fpath.read_text()
