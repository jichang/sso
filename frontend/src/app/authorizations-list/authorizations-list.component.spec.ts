import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { AuthorizationsListComponent } from './authorizations-list.component';

describe('AuthorizationsListComponent', () => {
  let component: AuthorizationsListComponent;
  let fixture: ComponentFixture<AuthorizationsListComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ AuthorizationsListComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AuthorizationsListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should be created', () => {
    expect(component).toBeTruthy();
  });
});
