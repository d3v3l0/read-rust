require "xml"

class Sitemap::Show < BrowserAction
  include Auth::AllowGuests

  get "/sitemap.xml" do
    xml sitemap
  end

  def sitemap
    XML.build(indent: "  ", encoding: "UTF-8") do |xml|
      xml.element("urlset", xmlns: "http://www.sitemaps.org/schemas/sitemap/0.9") do
        xml.element("url") do
          xml.element("loc") { xml.text Home::Index.url }
          xml.element("changefreq") { xml.text "daily" }
          xml.element("priority") { xml.text "0.8" }
        end

        xml.element("url") do
          xml.element("loc") { xml.text About::Show.url }
          xml.element("changefreq") { xml.text "weekly" }
        end

        xml.element("url") do
          xml.element("loc") { xml.text Submit::Show.url }
          xml.element("changefreq") { xml.text "weekly" }
        end

        xml.element("url") do
          xml.element("loc") { xml.text Creators::Index.url }
          xml.element("changefreq") { xml.text "weekly" }
        end

        CategoryQuery.new.each do |category|
          xml.element("url") do
            xml.element("loc") { xml.text Categories::Show.with(category.slug).url }
            xml.element("changefreq") { xml.text "daily" }
            xml.element("priority") { xml.text "0.7" }
          end
        end

        xml.element("url") do
          xml.element("loc") { xml.text Tags::Index.url }
          xml.element("changefreq") { xml.text "daily" }
          xml.element("priority") { xml.text "0.7" }
        end

        TagQuery.with_posts.each do |tag|
          xml.element("url") do
            xml.element("loc") { xml.text Tags::Show.with(tag).url }
            xml.element("changefreq") { xml.text "daily" }
            xml.element("priority") { xml.text "0.6" }
          end
        end
      end
    end
  end
end
