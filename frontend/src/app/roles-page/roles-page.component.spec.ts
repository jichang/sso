import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RolesPageComponent } from './roles-page.component';

describe('RolesPageComponent', () => {
  let component: RolesPageComponent;
  let fixture: ComponentFixture<RolesPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ RolesPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RolesPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
