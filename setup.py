from setuptools import setup

setup(
    name='socha',
    version='0.9.0',
    packages=['src', 'src.socha', 'src.socha.api', 'src.socha.api.plugin', 'src.socha.api.protocol',
              'src.socha.api.networking'],
    url='https://github.com/FalconsSky/Software-Challenge-Python-Client',
    license='GNU Lesser General Public License v3 (LGPLv3)',
    author='FalconsSky',
    author_email='stu222782@mail.uni-kiel.de',
    description='This is the package for the Software-Challenge Germany 2023. This Season the game will be \'Hey, '
                'danke für den Fisch\' a.k.a. \'Penguins\' in short. '
)
