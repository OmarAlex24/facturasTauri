import NavBar from "./common/NavBar"
import { ReactNode } from "react";

interface LayoutProps {
  children: ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  return (
    <div className="flex h-screen w-screen ">
      <NavBar />
      <main className="flex-grow overflow-auto p-5 text-center flex">
        {children}
      </main>
    </div>
  );
};

export default Layout;