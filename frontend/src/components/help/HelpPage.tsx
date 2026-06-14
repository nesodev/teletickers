import Sidebar from '../ui/Sidebar';
import Header from '../ui/Header';
import Footer from '../ui/Footer';
import Help from './Help';

export default function HelpPage() {
  return (
    <>
      <Sidebar />
      <div className="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-300">
        <Header />
        <Help />
        <Footer />
      </div>
    </>
  );
}