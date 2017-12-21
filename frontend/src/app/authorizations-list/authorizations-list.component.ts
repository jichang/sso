import { Component, Input } from '@angular/core';
import { Authorization } from '../authorization-model.service';

@Component({
  selector: 'authorizations-list',
  templateUrl: './authorizations-list.component.html',
  styleUrls: ['./authorizations-list.component.css']
})
export class AuthorizationsListComponent {
  @Input() authorizations: Authorization[];
}
