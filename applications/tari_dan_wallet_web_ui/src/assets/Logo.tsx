//  Copyright 2022. The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

interface LogoProps {
  width?: string;
  height?: string;
  fill?: string;
}

const Logo: React.FC<LogoProps> = ({
  width = '100%',
  height = 'auto',
  fill = 'black',
}) => (
  <svg
    width={width}
    height={height}
    viewBox="0 -5 350 60"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      d="M197.896 8.008H204.592C205.712 8.008 206.792 8.184 207.832 8.53599C208.872 8.87199 209.792 9.39199 210.592 10.096C211.392 10.8 212.032 11.688 212.512 12.76C212.992 13.816 213.232 15.064 213.232 16.504C213.232 17.96 212.952 19.224 212.392 20.296C211.848 21.352 211.136 22.232 210.256 22.936C209.392 23.624 208.424 24.144 207.352 24.496C206.296 24.832 205.256 25 204.232 25H197.896V8.008ZM203.176 22.264C204.12 22.264 205.008 22.16 205.84 21.952C206.688 21.728 207.424 21.392 208.048 20.944C208.672 20.48 209.16 19.888 209.512 19.168C209.88 18.432 210.064 17.544 210.064 16.504C210.064 15.48 209.904 14.6 209.584 13.864C209.264 13.128 208.816 12.536 208.24 12.088C207.68 11.624 207.008 11.288 206.224 11.08C205.456 10.856 204.608 10.744 203.68 10.744H200.92V22.264H203.176Z"
      fill={fill}
    />
    <path
      d="M221.603 8.008H224.219L231.539 25H228.083L226.499 21.112H219.131L217.595 25H214.211L221.603 8.008ZM225.443 18.52L222.827 11.608L220.163 18.52H225.443Z"
      fill={fill}
    />
    <path
      d="M233.427 8.008H237.435L245.667 20.632H245.715V8.008H248.739V25H244.899L236.499 11.968H236.451V25H233.427V8.008Z"
      fill={fill}
    />
    <path
      d="M196 30.008H199.312L202.504 42.248H202.552L206.368 30.008H209.344L213.088 42.248H213.136L216.472 30.008H219.544L214.6 47H211.72L207.784 34.064H207.736L203.8 47H201.016L196 30.008Z"
      fill={fill}
    />
    <path
      d="M226.337 30.008H228.953L236.273 47H232.817L231.233 43.112H223.865L222.329 47H218.945L226.337 30.008ZM230.177 40.52L227.561 33.608L224.897 40.52H230.177Z"
      fill={fill}
    />
    <path
      d="M238.162 30.008H241.186V44.264H248.41V47H238.162V30.008Z"
      fill={fill}
    />
    <path
      d="M250.607 30.008H253.631V44.264H260.855V47H250.607V30.008Z"
      fill={fill}
    />
    <path
      d="M263.052 30.008H274.308V32.744H266.076V36.92H273.876V39.656H266.076V44.264H274.74V47H263.052V30.008Z"
      fill={fill}
    />
    <path
      d="M281.204 32.744H275.996V30.008H289.436V32.744H284.228V47H281.204V32.744Z"
      fill={fill}
    />
    <path
      d="M115.148 30.8698L118.671 19.301L122.194 30.8698H115.148ZM114.869 8.29834L101.728 47.1145H110.134L112.908 38.14H124.434L127.208 47.1145H135.614L122.472 8.29834H114.869Z"
      fill={fill}
    />
    <path d="M176.445 8H184.258V46.8162H176.445V8Z" fill={fill} />
    <path
      d="M147.87 24.6427V15.2703H154.515C157.546 15.2703 159.147 16.8907 159.147 19.9569C159.147 23.0225 157.546 24.6427 154.515 24.6427H147.87ZM155.777 31.8769C162.953 31.5006 167.069 27.156 167.069 19.9569C167.069 12.4699 162.477 8 154.787 8H140.057V46.8159H147.87V34.6857L158.571 46.8159H168.634L155.354 31.8991L155.777 31.8769Z"
      fill={fill}
    />
    <path
      d="M82.5313 47.1145H90.3446V15.5689H103.461V8.29834H69.4141V15.5689H82.5313V47.1145Z"
      fill={fill}
    />
    <path
      d="M49.489 18.4517L49.4826 25.1813L9.9661 15.0154L23.3114 6.31705L49.489 18.4517ZM25.5188 46.7641L25.5086 24.732L46.5851 30.1605L25.5188 46.7641ZM20.0033 44.6529L5.51421 28.4097L5.50562 19.5436L19.9803 23.3081L20.0033 44.6529ZM0 14.9387L0.00212257 30.5344L22.7523 56L54.9548 30.6016L55 14.9151L22.826 0L0 14.9387Z"
      fill={fill}
    />
  </svg>
);

export default Logo;
