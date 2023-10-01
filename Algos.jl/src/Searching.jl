module Searching

function linear(items::Vector, item)::Union{Nothing,Integer}
    for i in 1:length(items)
        if items[i] == item
            return i
        end
    end
    return nothing
end

export linear

end
