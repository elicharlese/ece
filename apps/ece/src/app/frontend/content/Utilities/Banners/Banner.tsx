import React from 'react';
import './Banner.css';

interface BannerProps {
  title: string;
  subtitle?: string;
  backgroundImage: string;
}

const Banner: React.FC<BannerProps> = ({ title, subtitle, backgroundImage }) => {
  return (
    <div className="banner" style={{ backgroundImage: `url(${backgroundImage})` }}>
      <div className="banner-content">n        <h1 className="banner-title">{title}</h1>
        {subtitle && <h2 className="banner-subtitle">{subtitle}</h2>}
      </div>
    </div>
  );
};

export default Banner;