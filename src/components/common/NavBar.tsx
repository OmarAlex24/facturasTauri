import { Folder, Home, Settings } from 'lucide-react';
import { Link } from 'react-router-dom';

const NavBar = () => {
  return (
    <nav className="bg-gray-50 p-11 flex flex-col ">
      <div>
        <h1 className="text-2xl font-bold mb-8">Bienvenido!</h1>
        <div className="flex-grow">
          <ul className="space-y-2">
            {[
              { icon: <Home className="w-5 h-5" />, label: 'Inicio', route: '/' },
              { icon: <Folder className="w-5 h-5" />, label: 'XML', route: '/xmlReader' },
              { icon: <Settings className="w-5 h-5" />, label: 'Configuración', route: '/settings' },
            ].map((item, index) => (
              <Link to={item.route} className='flex hover:bg-gray-200 rounded'>
                <li key={index} className="flex items-center space-x-2 p-2">
                  {item.icon}
                  <span>{item.label}</span>
                </li>

              </Link>
            ))}
          </ul>
        </div>
      </div>

    </nav>
  );
}

export default NavBar;