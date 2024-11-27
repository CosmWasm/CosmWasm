// This code is @generated by cw-schema-codegen. Do not modify this manually.

{% if add_imports %}
import { z } from 'zod';
{% endif %}

/**
{% for doc in docs %}
    * {{ doc }}
{% endfor %}
 */

const {{ name }}Schema = 
{% match ty %}
    {% when TypeTemplate::Unit %}
        z.null()
    {% when TypeTemplate::Tuple with (types) %}
        z.tuple([{{ types|join(", ") }}])
    {% when TypeTemplate::Named with { fields } %}
        z.object({
            {% for field in fields %}
                /**
                {% for doc in field.docs %}
                    * {{ doc }}
                {% endfor %}
                 */

                {{ field.name }}: {{ field.ty }},
            {% endfor %}
        })
{% endmatch %}
;

type {{ name }} = z.infer<typeof {{ name }}Schema>;

export { {{ name }}, {{ name }}Schema };
