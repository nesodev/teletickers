import ApolloWrapper from '../providers/ApolloWrapper';
import ProfilePage from './ProfilePage';
import Sidebar from '../ui/Sidebar';
import Header from '../ui/Header';
import Footer from '../ui/Footer';

export default function Profile() {
  return (
    <>
      <Sidebar />
      <div className="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-300">
        <Header />
        <ApolloWrapper>
          <ProfilePage />
        </ApolloWrapper>
        <Footer />
      </div>
    </>
  );
}