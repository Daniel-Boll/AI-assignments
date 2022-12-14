import { ThemeSwitcherIcon } from '@ui/utility';
import Link from 'next/link';

function Header() {
  return (
    <>
      <div className="navbar sticky top-0 z-10 w-full h-fit bg-base-300/40 backdrop-blur-md rounded-lg">
        <div className="navbar-start"></div>
        <div className="navbar-center">
          <Link className="btn btn-ghost normal-case text-xl" href="/">
            AI Assignments
          </Link>
        </div>
        <div className="navbar-end">
          <ThemeSwitcherIcon />
        </div>
      </div>
    </>
  );
}

export { Header };
