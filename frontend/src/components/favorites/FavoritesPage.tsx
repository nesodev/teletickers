import ApolloWrapper from '../providers/ApolloWrapper';
import Favorites from './Favorites';
import Sidebar from '../ui/Sidebar';
import Header from '../ui/Header';
import Footer from '../ui/Footer';

export default function FavoritesPage() {
  return (
    <>
      <Sidebar />
      <div className="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-300">
        <Header />
        <ApolloWrapper>
          <Favorites />
        </ApolloWrapper>
        <Footer />
      </div>
    </>
  );
}