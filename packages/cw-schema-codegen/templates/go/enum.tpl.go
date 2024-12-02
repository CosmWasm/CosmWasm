// This code is @generated by cw-schema-codegen. Do not modify this manually.

{% if add_package %}
package cwcodegen

import (
    "github.com/cosmos/cosmos-sdk/types/address"
)
{% endif %}

{% for variant in variants %}
{% match variant.ty %}
{% when TypeTemplate::Unit %}
type {{ name }}{{ variant.name }} struct{}
{% when TypeTemplate::Tuple(types) %}
type {{ name }}{{ variant.name }} []interface{}
{% when TypeTemplate::Named { fields } %}
type {{ name }}{{ variant.name }} struct {
    {% for field in fields %}
	{% for doc in docs %}
	// {{ doc }}
	{% endfor %}
	{{ field.name }} {{ field.ty }} `json:"{{ field.rename }}"`
    {% endfor %}
}
{% endmatch %}
{% endfor %}

{% for doc in docs %}
	// {{ doc }}
{% endfor %}
type {{ name }} struct {
{% for variant in variants %}
{% for doc in docs %}
    // {{ doc }}
{% endfor %}
    {{ variant.name}} {{ name }}{{ variant.name }} `json:"{{ variant.rename }}"`
{% endfor %}
}
