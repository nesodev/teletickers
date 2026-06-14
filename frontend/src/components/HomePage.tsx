import ApolloWrapper from './providers/ApolloWrapper';
import EventListWithSections from './events/EventListWithSections';
import Sidebar from './ui/Sidebar';
import AdvancedSearchBar from './ui/AdvancedSearchBar';
import BannerCarousel from './ui/BannerCarousel';
import Header from './ui/Header';
import Footer from './ui/Footer';

export default function HomePage() {
  return (
    <>
      <Sidebar />
      <div className="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-300">
        <Header />
        <ApolloWrapper>
          <div className="container mx-auto px-4 py-8">
            <BannerCarousel />
            <AdvancedSearchBar />
            <EventListWithSections />
          </div>
        </ApolloWrapper>
        <Footer />
      </div>
    </>
  );
}