import Sidebar from '../ui/Sidebar';
import Header from '../ui/Header';
import Footer from '../ui/Footer';
import AccountSettings from './AccountSettings';

export default function AccountSettingsPage() {
  return (
    <>
      <Sidebar />
      <div className="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-300">
        <Header />
        <AccountSettings />
        <Footer />
      </div>
    </>
  );
}