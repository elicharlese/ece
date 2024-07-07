import React from 'react';
import styles from '../styles/Contribute.module.scss';

interface Startup {
  id: string;
  title: string;
  subtitle: string;
  description: string;
  bulletPoints: string[];
}

const startups: Startup[] = [
  {
    id: '1',
    title: 'CEC',
    subtitle: 'Creative Ecommerce Co.',
    description: 'Creative Ecommerce Co. is a full-service e-commerce platform specializing in containerization. Our mission is to empower businesses of all sizes with cutting-edge technology, providing a seamless and scalable solution for their online operations.',
    bulletPoints: [
      'Efficiency: Optimize resource utilization for lightning-fast performance.',
      'Scalability: Seamlessly scale your online store to meet growing demands.',
      'Security: Enhance data security through robust containerization.',
      'Portability: Enjoy easy migration and consistent performance across environments.'
    ]
  },
  {
    id: '2',
    title: 'TS',
    subtitle: 'TerraSolstice',
    description: 'TerraSolstice is a forward-thinking company committed to transforming urban landscapes. Our products are designed to harness the power of the sun, optimizing energy usage, and promoting sustainability in smart city development.',
    bulletPoints: [
      'Solar-Powered Hardware: Cutting-edge hardware solutions tailored for smart city infrastructure.',
      'Open-Source Blockchain: A robust blockchain architecture for secure and transparent data management.',
      'Data Optimization: Advanced algorithms for optimizing environmental data, ensuring efficiency.'
    ]
  },
  {
    id: '3',
    title: 'TDT',
    subtitle: 'Tradent',
    description: 'Tradent is a cutting-edge cybersecurity platform that specializes in preserving identity integrity, monitoring assets in real and virtual spaces, and fortifying authentication and data collection practices.',
    bulletPoints: [
      'Identity Integrity: Advanced AI recognition and secure space-time paths to ensure the integrity of digital identities.',
      'Real/Virtual Asset Monitoring: Comprehensive monitoring of both real and virtual assets guarantees a holistic approach to cybersecurity.',
      'Blockchain Security: Fortified with blockchain technology, Tradent ensures a secure and tamper-proof environment for sensitive data.',
      'API Integration: Seamlessly integrate Tradent\'s robust cybersecurity solutions into your existing systems with our comprehensive API documentation.'
    ]
  },
  {
    id: '4',
    title: 'OMNI',
    subtitle: 'OMNI',
    description: 'OMNI is where reality, augmented reality, and the metaverse converge to create a groundbreaking shared media-verse experience.',
    bulletPoints: [
      'Shared Reality Streaming: Share real-life experiences in real-time with friends and followers.',
      'Augmented Reality Enhancements: Elevate your streams with digital overlays and interactive elements.',
      'Metaverse Integration: Seamlessly transition between shared reality and immersive digital worlds.',
      'Customizable Avatars and Environments: Personalize your presence with unique avatars and environments.'
    ]
  },
  {
    id: '5',
    title: 'SPECTRA',
    subtitle: 'Revolutionary 3D Spatial Trading Software',
    description: 'SPECTRA provides an immersive experience, offering unparalleled insights and dynamic trading opportunities through the integration of spatial data layers and maps into market analysis.',
    bulletPoints: [
      'Spatial Insights: Explore markets in three dimensions, uncovering hidden correlations and trends.',
      'Dynamic Heatmaps: Visualize market activity in real-time with dynamic spatial heatmaps.',
      'Collaborative Community: Engage with a vibrant community of traders, sharing spatial insights and trading strategies.',
      'Real-time Market Visualizations: Stay ahead of the curve with up-to-the-minute visualizations of market movements.'
    ]
  },
  {
    id: '6',
    title: 'STOKE',
    subtitle: 'Transforming the Cryptocurrency Trading Experience',
    description: 'STOKE empowers retail traders by fostering a global community that collaborates, shares insights, and makes collective decisions.',
    bulletPoints: [
      'Micro-Fund Management: Create and join micro-funds with other traders to pool resources and make collective investment decisions.',
      'Community Scripts and Signals: Share and explore community-driven trading scripts and signals for informed decision-making.',
      'Educational Resources: Access a wealth of educational materials to enhance trading skills and knowledge.',
      'Transparent Governance: STOKE is committed to transparency, providing clear governance and decision-making processes within the community.'
    ]
  },
  {
    id: '7',
    title: 'DASHED',
    subtitle: 'Revolutionary Personal Operating System',
    description: 'DASHED unifies your digital world seamlessly, empowering you with a consistent and personalized interface flow across all your devices.',
    bulletPoints: [
      'Modular Widgets: Tailor your digital experience with a wide array of customizable widgets.',
      'Cross-Device Integration: Achieve a unified flow across all your devices effortlessly.',
      'Seamless Connectivity: DASHED connects your streams of thought and work like never before.',
      'Developer-Friendly: Comprehensive API documentation for creating your own custom modules.'
    ]
  },
  {
    id: '8',
    title: 'GSL',
    subtitle: 'Mindful AI Integration',
    description: 'GSL is redefining the relationship between humans and technology through mindful AI integration, enhancing well-being and promoting mindfulness in the digital age.',
    bulletPoints: [
      'Open-Source LLM Model: GSL’s Lion Language Model powers our AI solutions, fostering meditative reflection and hyper-autocatalytic cycles of automation.',
      'Hardware Solutions: Immerse yourself in our range of cutting-edge hardware, from AR Headsets to Tablet-like Hard Wallets for cryptocurrency.',
      'Flashlabor Service: Swiftly trigger and manage Intelligent Digital Workers for rapid, on-demand task completion.'
    ]
  }
];

const handleApplyClick = (startupTitle: string) => {
  alert(`You are about to invest in ${startupTitle}! This will prompt a purchase using your wallet.`);
  // Implement wallet interaction logic here
};

const Startups: React.FC = () => {
  return (
    <div className={styles.container}>
      {startups.map((startup) => (
        <div key={startup.id} className={styles.item}>
          <div className={styles.imageBox}>
            {/* Image upload or placeholder */}
          </div>
          <div className={styles.textContainer}>
            <div className={styles.title}>{startup.title}</div>
            <span className={styles.subtitle}>{startup.subtitle}</span>
            <p className={styles.description}>{startup.description}</p>
            <ul className={styles.bulletPoints}>
              {startup.bulletPoints.map((point, index) => (
                <li key={index}>{point}</li>
              ))}
            </ul>
            <button
              className={styles.applyButton}
              onClick={() => handleApplyClick(startup.title)}
            >
              Apply Here
            </button>
          </div>
        </div>
      ))}
    </div>
  );
};

export default Startups;