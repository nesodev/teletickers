import ApolloWrapper from './providers/ApolloWrapper';
import EventListWithSections from './events/EventListWithSections';
import Sidebar from './ui/Sidebar';
import AdvancedSearchBar from './ui/AdvancedSearchBar';
import BannerCarousel from './ui/BannerCarousel';
import Header from './ui/Header';
import Footer from './ui/Footer';
import BottomNav from './ui/BottomNav';

export default function HomePage() {
  return (
    <>
      {/* Sidebar solo visible en desktop */}
      <div className="hidden md:block">
        <Sidebar />
      </div>

      <div className="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-300">
        <Header />
        <ApolloWrapper>
          {/* En mobile: sin padding lateral del sidebar */}
          <div className="container mx-auto px-4 py-4 md:py-8 pb-24 md:pb-8">
            <BannerCarousel />
            <AdvancedSearchBar />
            <EventListWithSections />
          </div>
        </ApolloWrapper>
        {/* Footer solo en desktop */}
        <div className="hidden md:block">
          <Footer />
        </div>
      </div>

      {/* Bottom nav solo en mobile */}
      <BottomNav />
    </>
  );
}